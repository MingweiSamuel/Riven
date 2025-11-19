#![allow(dead_code, unused_imports)]

#[cfg(not(target_family = "wasm"))]
use std::env::var as env_var;
use std::future::Future;
use std::sync::OnceLock;

use futures::try_join;
use riven::consts::{PlatformRoute, QueueType, RegionalRoute, ValPlatformRoute};
use riven::{RiotApi, RiotApiConfig};
#[cfg(not(target_family = "wasm"))]
pub use tokio_shared_rt::test as riven_test;
#[cfg(target_family = "wasm")]
pub use wasm_bindgen_test::wasm_bindgen_test as riven_test;
#[cfg(target_family = "wasm")]
#[allow(non_upper_case_globals)]
pub fn env_var<K: AsRef<str>>(key: K) -> Result<String, std::env::VarError> {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        type Process;
        #[wasm_bindgen(thread_local_v2)]
        static process: Process;

        type Env;

        #[wasm_bindgen(method, getter)]
        fn env(this: &Process) -> Env;

        #[wasm_bindgen(method, structural, indexing_getter)]
        fn get(this: &Env, field: &str) -> Option<String>;
    }

    process
        .with(Process::env)
        .get(key.as_ref())
        .ok_or(std::env::VarError::NotPresent)
}

#[macro_export]
macro_rules! rassert {
    ( $x:expr ) => {
        {
            if $x { Ok(()) } else { Err(stringify!($x)) }?
        }
    };
    ( $x:expr, $format:expr $(, $arg:expr)* ) => {
        {
            if $x { Ok(()) } else { Err( format!($format $(, $arg )* ) ) }?
        }
    };
}

#[macro_export]
macro_rules! rassert_eq {
    ( $a:expr, $b:expr ) => {
        {
            let a = &$a;
            let b = &$b;
            rassert!(a == b, "should be equal: {:?}, {:?}", a, b)
        }
    };
    ( $a:expr, $b:expr, $format:expr $(, $arg:expr)* ) => {
        rassert!($a == $b, $format $(, $arg )* )
    };
}

#[macro_export]
macro_rules! rassert_ne {
    ( $a:expr, $b:expr ) => { rassert!($a != $b) };
    ( $a:expr, $b:expr, $format:expr $(, $arg:expr)* ) => {
        rassert!($a != $b, $format $(, $arg )* )
    };
}

static RIOT_API: OnceLock<RiotApi> = OnceLock::new();
pub fn riot_api() -> &'static RiotApi {
    RIOT_API.get_or_init(|| {
        // Initialize logger here, as a convenient trigger spot.
        #[cfg(not(target_family = "wasm"))]
        {
            #[cfg(not(feature = "tracing"))]
            env_logger::init();
            #[cfg(feature = "tracing")]
            tracing_subscriber::fmt::init();
        }

        #[cfg(target_family = "wasm")]
        console_log::init_with_level(log::Level::Info).unwrap();

        let api_key = env_var("RGAPI_KEY")
            .ok()
            .or_else(|| {
                use std::iter::FromIterator;

                let path =
                    std::path::PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "../apikey.txt"]);
                std::fs::read_to_string(path).ok()
            })
            .expect("Failed to find RGAPI_KEY env var or apikey.txt.");
        RiotApi::new(RiotApiConfig::with_key(api_key.trim()).preconfig_burst())
    })
}

/// Get recent Challenger matches and check that they parse as valid.
pub async fn league_v4_match_v5_latest_combo(route: PlatformRoute) -> Result<(), String> {
    const NUM_MATCHES: usize = 10;

    let challenger_future = riot_api()
        .league_v4()
        .get_challenger_league(route, QueueType::RANKED_SOLO_5x5);
    let challenger_league = challenger_future
        .await
        .map_err(|e| format!("Failed to get challenger league: {}", e))?;

    let Some(queue) = challenger_league.queue else {
        assert!(challenger_league.entries.is_empty());
        eprintln!("Off-season, challenger league is empty.");
        return Ok(());
    };

    if QueueType::RANKED_SOLO_5x5 != queue {
        return Err(format!("Unexpected `queue`: {:?}", queue));
    }
    if challenger_league.entries.is_empty() {
        return Err("Challenger league is unexpectedly empty!".to_owned());
    }

    let match_ids_futures = challenger_league
        .entries
        .iter()
        .take(5)
        .map(|entry| async move {
            let match_ids_future = riot_api().match_v5().get_match_ids_by_puuid(
                route.to_regional(),
                &entry.puuid,
                Some(5),
                None,
                None,
                None,
                None,
                None,
            );
            let match_ids = match_ids_future
                .await
                .map_err(|e| format!("Failed to find summoner match IDs: {}", e))?;
            Ok(match_ids) as Result<_, String>
        });

    let match_ids = futures::future::try_join_all(match_ids_futures).await?;

    let mut match_ids: Vec<String> = match_ids.into_iter().flatten().collect();
    match_ids.sort_unstable_by(|a, b| a.cmp(b).reverse()); // Sort descending, so latest are first.

    let _ = try_join!(
        match_v5_get(route.to_regional(), match_ids.iter().take(NUM_MATCHES)),
        match_v5_get_timeline(route.to_regional(), match_ids.iter().take(NUM_MATCHES)),
    )?;

    Ok(())
}

/// Check that the given TFT matches parse as valid.
pub async fn tft_match_v1_get(
    route: RegionalRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = riot_api().tft_match_v1().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get tft match {}: {}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;

        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err(format!(
                "Match {} should have participants (metadata).",
                matche
            ));
        }
        // Due to Riot's addition of bots to TFT, this test is not possible anymore
        if m.metadata.participants.len() != m.info.participants.len() {
            eprintln!(
                "Match {} participants do not line up with participant UUIDs.",
                matche
            );
        }
        if m.info.participants.is_empty() {
            return Err(format!("Match {} should have participants (info).", matche));
        }
        Ok(())
    });
    futures::future::try_join_all(futures).await?;
    Ok(())
}

/// Check that the given LoL matches parse as valid.
pub async fn match_v5_get(
    route: RegionalRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = riot_api().match_v5().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;

        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err(format!("Match {} should have participants.", matche));
        }
        if m.metadata.participants.len() != m.info.participants.len() {
            // Sometimes only returns match IDs for one team? JP1_391732436
            // Do not return error.
            eprintln!(
                "Match {} participants do not line up with participant UUIDs.",
                matche
            );
        }
        for participant in &m.info.participants {
            participant
                .champion()
                .map_err(|e| format!("Failed to determine match {} champion: {}", matche, e))?;
        }

        {
            let game_id = m.info.game_id;
            if 0 == game_id {
                eprintln!(
                    "Match {} `game_id` is zero, skipping remaining tests (see https://github.com/RiotGames/developer-relations/issues/898).",
                    matche
                );
                return Ok(());
            } else if matche[(matche.find('_').unwrap() + 1)..] != game_id.to_string() {
                return Err(format!(
                    "Match {} timeline number ID should match `game_id` {}.",
                    matche, game_id
                ));
            }
        }

        if m.info.teams.is_empty() {
            return Err(format!("Match {} should have teams.", matche));
        }
        Ok(())
    });
    join_all_future_errs(futures).await
}

/// Check that the given LoL match timelines parse as valid.
pub async fn match_v5_get_timeline(
    route: RegionalRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = riot_api().match_v5().get_timeline(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match timeline {}: {}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;
        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err(format!("Match {} should have participants.", matche));
        }
        if let Some(game_id) = m.info.game_id {
            if 0 == game_id {
                eprintln!("Match {} timeline `game_id` is zero (see https://github.com/RiotGames/developer-relations/issues/898).", matche);
            } else if matche[(matche.find('_').unwrap() + 1)..] != game_id.to_string() {
                return Err(format!(
                    "Match {} timeline number ID should match `game_id` {}.",
                    matche, game_id
                ));
            }
        }
        if m.info.frames.is_empty() {
            return Err(format!("Match {} timeline should have frames.", matche));
        }
        Ok(())
    });
    join_all_future_errs(futures).await
}

/// Test getting a ranked leaderboard.
pub async fn val_content_ranked(route: ValPlatformRoute) -> Result<(), String> {
    let p = riot_api()
        .val_content_v1()
        .get_content(route, Some("zh-CN")); // Idk
    let contents = p
        .await
        .map_err(|e| format!("Failed to get content: {}", e))?;

    // Find the LAST active act, via `.rev().find(...)`.
    // Added filter when parent id is 0000... as there are multiple that are active, the last active seems to be episode 5
    // Not sure if this a bandaid fix
    let act = contents
        .acts
        .iter()
        .rev()
        .find(|act| {
            act.is_active
                && act.parent_id != Some("00000000-0000-0000-0000-000000000000".to_string())
        })
        .ok_or(format!("No active acts of {} found.", contents.acts.len()))?;

    println!("ACT {:?}", act);

    let p = riot_api()
        .val_ranked_v1()
        .get_leaderboard(route, &act.id, None, None);
    let leaderboard = p
        .await
        .map_err(|e| format!("Failed to get act leaderboard: {} {}", act.id, e))?
        .ok_or(format!(
            "Act leaderboard was not found: {} {}.",
            act.id, act.name
        ))?;

    if leaderboard.act_id.is_empty() {
        eprintln!("Leaderboard has empty `act_id`, continuing anyway.");
    } else {
        rassert_eq!(act.id, leaderboard.act_id);
    }

    for (i, p) in leaderboard.players.iter().take(10).enumerate() {
        rassert_eq!(i + 1, p.leaderboard_rank as usize);
        println!(
            "{:>2}: {:>4}   {:<22} ({} wins)",
            p.leaderboard_rank,
            p.ranked_rating,
            format!(
                "{}#{}",
                p.game_name.as_deref().unwrap_or("<NONE>"),
                p.tag_line.as_deref().unwrap_or("<NONE>")
            ),
            p.number_of_wins
        );
    }

    Ok(())
}

/// Get recent Challenger matches and check that they parse as valid.
pub async fn val_match_v1_latest(route: ValPlatformRoute) -> Result<(), String> {
    // Val rate limits are super low.
    const NUM_MATCHES: usize = 5;

    let recent_matches = riot_api()
        .val_match_v1()
        .get_recent(route, "competitive")
        .await
        .map_err(|e| format!("Failed to get recent Valorant matches: {}", e))?;

    val_match_v1_get(route, &recent_matches.match_ids[..NUM_MATCHES]).await
}

/// Check that the given Valorant matches parse as valid.
pub async fn val_match_v1_get(
    route: ValPlatformRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = riot_api().val_match_v1().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get val match {}: {}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;

        // TODO(mingwei): Check the match a bit.
        let _ = m;

        Ok(())
    });
    join_all_future_errs(futures).await
}

/// Gets replays for a given Riot ID.
pub async fn match_v5_get_replay(
    route: RegionalRoute,
    game_name: &str,
    tag_line: &str,
) -> Result<(), String> {
    let account = riot_api()
        .account_v1()
        .get_by_riot_id(route, game_name, tag_line)
        .await
        .map_err(|e| {
            format!(
                "Failed to get account `{}#{}` by riot ID: {}",
                game_name, tag_line, e
            )
        })?
        .ok_or_else(|| format!("Account `{}#{}` not found!", game_name, tag_line))?;

    let replays = riot_api()
        .match_v5()
        .get_replay(route, &account.puuid)
        .await
        .map_err(|e| {
            format!(
                "Failed to get replays for account `{}#{}`: {}",
                game_name, tag_line, e
            )
        })?;

    assert_eq!(replays.total as usize, replays.match_file_ur_ls.len());
    println!(
        "{} replays:\n{:#?}",
        replays.total, replays.match_file_ur_ls
    );

    Ok(())
}

/// Joins all futures and keeps ALL error messages, separated by newlines.
pub async fn join_all_future_errs<T>(
    result_tasks: impl Iterator<Item = impl Future<Output = Result<T, String>>>,
) -> Result<(), String> {
    futures::future::join_all(result_tasks)
        .await
        .into_iter()
        .filter_map(Result::err)
        .reduce(|a, b| a + "\n" + &b)
        .map(Err)
        .unwrap_or(Ok(()))
}

#![allow(dead_code)]

use std::future::Future;
use std::sync::OnceLock;

use riven::consts::{PlatformRoute, QueueType, RegionalRoute};
use riven::{RiotApi, RiotApiConfig};

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
    ( $a:expr, $b:expr ) => { rassert!($a == $b) };
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
        env_logger::init();

        let api_key = std::env::var("RGAPI_KEY")
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
            let summoner_future = riot_api()
                .summoner_v4()
                .get_by_summoner_id(route, &entry.summoner_id);
            let summoner_info = summoner_future
                .await
                .map_err(|e| format!("Failed to find summoner info: {}", e))?;

            let match_ids_future = riot_api().match_v5().get_match_ids_by_puuid(
                route.to_regional(),
                &summoner_info.puuid,
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

    let _ = tokio::try_join!(
        match_v5_get(route.to_regional(), match_ids.iter().take(NUM_MATCHES)),
        match_v5_get_timeline(route.to_regional(), match_ids.iter().take(NUM_MATCHES)),
    )?;

    Ok(())
}

pub async fn tft_match_v1_get(
    route: RegionalRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = riot_api().tft_match_v1().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
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
        if m.metadata.participants.len() != m.info.participants.len() {
            return Err(format!(
                "Match {} participants do not line up with participant UUIDs.",
                matche
            ));
        }
        if m.info.participants.is_empty() {
            return Err(format!("Match {} should have participants (info).", matche));
        }
        Ok(())
    });
    futures::future::try_join_all(futures).await?;
    Ok(())
}

pub async fn match_v5_get(
    route: RegionalRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = riot_api().match_v5().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
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
        if m.info.teams.is_empty() {
            return Err(format!("Match {} should have teams.", matche));
        }
        Ok(())
    });
    join_all_future_errs(futures).await
}

pub async fn match_v5_get_timeline(
    route: RegionalRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = riot_api().match_v5().get_timeline(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
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
            if matche[(matche.find('_').unwrap() + 1)..] != game_id.to_string() {
                return Err(format!("Match {} number ID should match.", matche));
            }
        }
        if m.info.frames.is_empty() {
            return Err(format!("Match {} timleine should have frames.", matche));
        }
        Ok(())
    });
    join_all_future_errs(futures).await
}

/// Joins all futures and keeps ALL error messages, separated by newlines.
async fn join_all_future_errs<T>(
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

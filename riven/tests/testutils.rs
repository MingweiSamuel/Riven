#![allow(dead_code)]

use lazy_static::lazy_static;

use riven::consts::{PlatformRoute, QueueType, RegionalRoute};
use riven::{RiotApi, RiotApiConfig};

lazy_static! {
    pub static ref RIOT_API: RiotApi = {
        let api_key = std::env::var("RGAPI_KEY")
            .ok()
            .or_else(|| std::fs::read_to_string("apikey.txt").ok())
            .expect("Failed to find RGAPI_KEY env var or apikey.txt.");
        RiotApi::new(RiotApiConfig::with_key(api_key.trim()).preconfig_burst())
    };
}

pub async fn league_v4_match_v5_latest_combo(route: PlatformRoute) -> Result<(), String> {
    const NUM_MATCHES: usize = 10;

    let challenger_future = RIOT_API
        .league_v4()
        .get_challenger_league(route, QueueType::RANKED_SOLO_5x5);
    let challenger_league = challenger_future.await.map_err(|e| e.to_string())?;

    if &QueueType::RANKED_SOLO_5x5 != &challenger_league.queue {
        return Err(format!("Unexpected `queue`: {}", challenger_league.queue));
    }
    if challenger_league.entries.is_empty() {
        return Err("Challenger league is unexpectedly empty!".to_owned());
    }

    let match_ids_futures = challenger_league
        .entries
        .iter()
        .take(5)
        .map(|entry| async move {
            let summoner_future = RIOT_API
                .summoner_v4()
                .get_by_summoner_id(route, &entry.summoner_id);
            let summoner_info = summoner_future.await.map_err(|e| e.to_string())?;

            let match_ids_future = RIOT_API.match_v5().get_match_ids_by_puuid(
                route.to_regional(),
                &*summoner_info.puuid,
                Some(5),
                None,
                None,
                None,
                None,
                None,
            );
            let match_ids = match_ids_future.await.map_err(|e| e.to_string())?;
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
        let p = RIOT_API.tft_match_v1().get_match(route, matche);
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
            return Err("Match should have participants (metadata).".to_owned());
        }
        if m.metadata.participants.len() != m.info.participants.len() {
            return Err("Match participants do not line up with participant UUIDs.".to_owned());
        }
        if m.info.participants.is_empty() {
            return Err("Match should have participants (info).".to_owned());
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
        let p = RIOT_API.match_v5().get_match(route, matche);
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
            return Err("Match should have participants.".to_owned());
        }
        if m.metadata.participants.len() != m.info.participants.len() {
            return Err("Match participants do not line up with participant UUIDs.".to_owned());
        }
        for participant in &m.info.participants {
            participant
                .champion()
                .map_err(|e| format!("Failed to determine champion: {}", e))?;
        }
        if m.info.teams.is_empty() {
            return Err("Match should have teams.".to_owned());
        }
        Ok(())
    });
    futures::future::try_join_all(futures).await?;
    Ok(())
}

pub async fn match_v5_get_timeline(
    route: RegionalRoute,
    matches: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), String> {
    let futures = matches.into_iter().map(|matche| async move {
        let matche = matche.as_ref();
        let p = RIOT_API.match_v5().get_timeline(route, matche);
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
            return Err("Match should have participants.".to_owned());
        }
        if let Some(game_id) = m.info.game_id {
            if matche[(matche.find('_').unwrap() + 1)..] != game_id.to_string() {
                return Err("Match number ID should match.".to_owned());
            }
        }
        if m.info.frames.is_empty() {
            return Err("Match timleine should have frames.".to_owned());
        }
        Ok(())
    });
    futures::future::try_join_all(futures).await?;
    Ok(())
}

mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::EUW1;

static TFT_MATCHES: &[&str] = &[
    // https://github.com/MingweiSamuel/Riven/pull/62
    // https://github.com/MingweiSamuel/riotapi-schema/pull/43
    "EUW1_6786745342",
    // 2024-02-16
    "EUW1_6807630149",
    // 2025-01-31
    "EUW1_7288129746",
];

#[riven_test]
async fn tftmatchv1_get_list() -> Result<(), String> {
    tft_match_v1_get(ROUTE.to_regional(), TFT_MATCHES).await
}

// /// Don't have acecess to tft-status-v1.
// #[riven_test]
// async fn tftstatusv1_getplatformdata() -> Result<(), String> {
//     let p = riot_api().tft_status_v1().get_platform_data(ROUTE);
//     let _s = p.await.map_err(|e| e.to_string())?;
//     Ok(())
// }

#[riven_test]
async fn tftleaguev1_gettopratedladder() -> Result<(), String> {
    let p = riot_api()
        .tft_league_v1()
        .get_top_rated_ladder(ROUTE, QueueType::RANKED_TFT_TURBO);
    let l = p.await.map_err(|e| e.to_string())?;
    if l.len() < 10 {
        eprintln!(
            "Only {} top ranked players found! Is it the off-season?",
            l.len()
        );
    }
    Ok(())
}

/// Get top rated player, get some of their matches.
/// TODO(mingwei): Deduplicate with `tft_combo` in `tests_asia_jp.rs`.
#[riven_test]
async fn tft_combo() -> Result<(), String> {
    let top_players = riot_api()
        .tft_league_v1()
        .get_top_rated_ladder(ROUTE, QueueType::RANKED_TFT_TURBO);
    let top_players = top_players
        .await
        .map_err(|e| format!("Failed to get top players: {}", e))?;
    let Some(top_player_entry) = top_players.first() else {
        eprintln!("No top players found! Is it the off-season? Skipping remaining checks.");
        return Ok(());
    };
    let top_player = riot_api()
        .tft_summoner_v1()
        .get_by_puuid(ROUTE, &top_player_entry.puuid);
    let top_player = top_player
        .await
        .map_err(|e| format!("Failed to get summoner of top player: {}", e))?;
    println!("Top player has `puuid` {}.", top_player.puuid);
    let match_ids = riot_api().tft_match_v1().get_match_ids_by_puuid(
        ROUTE.to_regional(),
        &top_player.puuid,
        Some(10),
        None,
        None,
        None,
    );
    let match_ids = match_ids
        .await
        .map_err(|e| format!("Failed to get match IDs: {}", e))?;
    tft_match_v1_get(ROUTE.to_regional(), &*match_ids).await?;
    Ok(())
}

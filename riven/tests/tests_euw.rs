mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::EUW1;

// Champion Mastery tests

#[riven_test]
async fn championmastery_getscore_ma5tery() -> Result<(), String> {
    let sum = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "ma5tery");
    let sum = sum
        .await
        .map_err(|e| format!("Error getting summoner: {}", e))?
        .ok_or_else(|| "Failed to find summoner".to_owned())?;

    let p = riot_api()
        .champion_mastery_v4()
        .get_champion_mastery_score_by_puuid(ROUTE, &sum.puuid);
    let s = p
        .await
        .map_err(|e| format!("Error getting champion mastery score: {}", e))?;
    rassert!(
        (969..=1000).contains(&s),
        "Unexpected ma5tery score: {}.",
        s
    );
    Ok(())
}

#[riven_test]
async fn championmastery_getall_ma5tery() -> Result<(), String> {
    let sum = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "ma5tery");
    let sum = sum
        .await
        .map_err(|e| format!("Error getting summoner: {}", e))?
        .ok_or_else(|| "Failed to find summoner".to_owned())?;

    let p = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(ROUTE, &sum.puuid);
    let s = p
        .await
        .map_err(|e| format!("Error getting all champion masteries: {}", e))?;
    rassert!(s.len() >= 142, "Expected masteries: {}.", s.len());
    Ok(())
}

#[riven_test]
async fn spectator_v4_combo_test() -> Result<(), String> {
    spectator_v4_combo(ROUTE).await
}

#[riven_test]
async fn spectator_v5_combo_test() -> Result<(), String> {
    spectator_v5_combo(ROUTE).await
}

#[riven_test]
async fn spectator_tft_v5_combo_test() -> Result<(), String> {
    spectator_tft_v5_combo(ROUTE).await
}

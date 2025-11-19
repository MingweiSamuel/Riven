mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::EUW1;

// Champion Mastery tests

#[riven_test]
async fn championmastery_getscore_ma5tery() -> Result<(), String> {
    let account = riot_api()
        .account_v1()
        .get_by_riot_id(RegionalRoute::AMERICAS, "ma5tery", "EUW");
    let account = account
        .await
        .map_err(|e| format!("Error getting summoner: {}", e))?
        .ok_or_else(|| "Failed to find summoner".to_owned())?;

    let masteries = riot_api()
        .champion_mastery_v4()
        .get_champion_mastery_score_by_puuid(ROUTE, &account.puuid);
    let masteries = masteries
        .await
        .map_err(|e| format!("Error getting champion mastery score: {}", e))?;
    rassert!(
        (1002..=1100).contains(&masteries),
        "Unexpected ma5tery score: {}.",
        masteries
    );
    Ok(())
}

#[riven_test]
async fn championmastery_getall_ma5tery() -> Result<(), String> {
    let account = riot_api()
        .account_v1()
        .get_by_riot_id(RegionalRoute::AMERICAS, "ma5tery", "EUW");
    let account = account
        .await
        .map_err(|e| format!("Error getting summoner: {}", e))?
        .ok_or_else(|| "Failed to find summoner".to_owned())?;

    let masteries = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(ROUTE, &account.puuid);
    let masteries = masteries
        .await
        .map_err(|e| format!("Error getting all champion masteries: {}", e))?;
    rassert!(
        masteries.len() >= 142,
        "Expected masteries: {}.",
        masteries.len()
    );
    Ok(())
}

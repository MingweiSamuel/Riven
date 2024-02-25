mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::RU;

#[riven_test]
async fn summoner_leagues() -> Result<(), String> {
    let sum = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "d3atomiz3d");
    let sum = sum
        .await
        .map_err(|e| format!("Error getting summoner: {}", e))?
        .ok_or_else(|| "Failed to find summoner".to_owned())?;

    let p = riot_api()
        .league_v4()
        .get_league_entries_for_summoner(ROUTE, &sum.id);
    let s = p
        .await
        .map_err(|e| format!("Error getting league entries: {}", e))?;
    let _ = s;
    Ok(())
}

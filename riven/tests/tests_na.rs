mod testutils;
use core::option::Option::None;

use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::NA1;

#[riven_test]
async fn champion_getrotation() -> Result<(), String> {
    let p = riot_api().champion_v3().get_champion_info(ROUTE);
    let d = p.await.map_err(|e| e.to_string())?;
    let new_len = d.free_champion_ids_for_new_players.len();
    let free_len = d.free_champion_ids.len();
    let level = d.max_new_player_level;
    rassert!(new_len >= 10, "New len: {}", new_len);
    rassert!(free_len >= 15, "Free len: {}", free_len);
    rassert_eq!(10, level, "New player level: {}", level);
    Ok(())
}

#[riven_test]
async fn league_get_diamond() -> Result<(), String> {
    let p = riot_api().league_v4().get_league_entries(
        ROUTE,
        QueueType::RANKED_SOLO_5x5,
        Tier::DIAMOND,
        Division::IV,
        None,
    );
    let opt = p
        .await
        .map_err(|e| format!("Failed to get league entries: {}", e))?;
    if let Some(list) = opt {
        assert!(!list.is_empty(), "Returns 204 (`None`) when empty.");
    } else {
        eprintln!("Off-season, challenger league is empty.");
    }
    Ok(())
}

#[riven_test]
async fn league_get_challenger() -> Result<(), String> {
    let p = riot_api()
        .league_v4()
        .get_challenger_league(ROUTE, QueueType::RANKED_SOLO_5x5);
    let ll = p.await.map_err(|e| e.to_string())?;
    if ll.entries.is_empty() {
        eprintln!("Off-season, challenger league is empty.");
    }
    Ok(())
}

#[riven_test]
async fn leagueexp_get_challenger() -> Result<(), String> {
    let p = riot_api().league_exp_v4().get_league_entries(
        ROUTE,
        QueueType::RANKED_SOLO_5x5,
        Tier::CHALLENGER,
        Division::I,
        None,
    );
    let opt = p.await.map_err(|e| e.to_string())?;
    if let Some(list) = opt {
        assert!(!list.is_empty(), "Returns 204 (`None`) when empty.");
    } else {
        eprintln!("Off-season, challenger league is empty.");
    }
    Ok(())
}

#[riven_test]
async fn championmasteryv4_lugnutsk() -> Result<(), String> {
    let account =
        riot_api()
            .account_v1()
            .get_by_riot_id(RegionalRoute::AMERICAS, "LugnutsK", "000");
    let account = account
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "'LugnutsK' not found!".to_owned())?;
    let masteries = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(ROUTE, &account.puuid);
    let masteries = masteries.await.map_err(|e| e.to_string())?;
    rassert!(74 <= masteries.len());
    Ok(())
}

#[riven_test]
async fn championmasteryv4_getall_iamchanese123() -> Result<(), String> {
    let summoner =
        riot_api()
            .account_v1()
            .get_by_riot_id(ROUTE.to_regional(), "iamchanese123", "NA1");
    let summoner = summoner
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "'LugnutsK' not found!".to_owned())?;
    let masteries = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(ROUTE, &summoner.puuid);
    let masteries = masteries.await.map_err(|e| e.to_string())?;
    rassert_eq!(59, masteries.len());
    Ok(())
}

// Commented out, requires special API key.
// /// LOR
// #[riven_test]
// async fn async fn lor_ranked_get_leaderboards() -> Result<(), String> {
//     let future = riot_api().lor_ranked_v1().get_leaderboards(Region::AMERICAS);
//     let _leaderboard = future.await.map_err(|e| e.to_string())?;
//     Ok(())
// }

// CLASH

#[riven_test]
async fn clash_get_tournaments() -> Result<(), String> {
    let p = riot_api().clash_v1().get_tournaments(ROUTE);
    let tours = p.await.map_err(|e| e.to_string())?;
    if let Some(tour0) = tours.first() {
        let p = riot_api().clash_v1().get_tournament_by_id(ROUTE, tour0.id);
        let tour1 = p.await.map_err(|e| e.to_string())?;
        assert_eq!(Some(tour0.id), tour1.map(|t| t.id));
    }
    Ok(())
}

#[riven_test]
async fn clash_get_team_by_id_invalid() -> Result<(), String> {
    let p = riot_api()
        .clash_v1()
        .get_team_by_id(ROUTE, "00000000-0000-0000-0000-000000000000");
    let team = p.await.map_err(|e| e.to_string())?;
    assert!(team.is_none());
    Ok(())
}

// STATUS
#[riven_test]
async fn status() -> Result<(), String> {
    let p = riot_api().lol_status_v4().get_platform_data(ROUTE);
    let status = p.await.map_err(|e| e.to_string())?;
    let _ = status;
    Ok(())
}

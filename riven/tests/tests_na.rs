mod testutils;
use riven::consts::*;
use riven::models::summoner_v4::*;
use testutils::*;

fn validate_summoners(s1: Summoner, s2: Summoner) -> Result<(), String> {
    rassert_eq!(s1.name, s2.name, "Names didn't match {}.", "");
    rassert_eq!(s1.id, s2.id, "SummonerId didn't match {}.", "");
    rassert_eq!(
        s1.account_id,
        s2.account_id,
        "AccountId didn't match {}.",
        ""
    );
    Ok(())
}

const ROUTE: PlatformRoute = PlatformRoute::NA1;

// Summoner tests.

#[riven_test]
async fn summoner_double() -> Result<(), String> {
    let l1p = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "lug nuts k");
    let l2p = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "lugnuts k");
    let l1 = l1p
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "'lug nuts k' not found!".to_owned())?;
    let l2 = l2p
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "'lugnuts k' not found!".to_owned())?;
    validate_summoners(l1, l2)?;
    Ok(())
}

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
async fn leagueexp_get() -> Result<(), String> {
    let p = riot_api().league_exp_v4().get_league_entries(
        ROUTE,
        QueueType::RANKED_SOLO_5x5,
        Tier::CHALLENGER,
        Division::I,
        None,
    );
    let d = p.await.map_err(|e| e.to_string())?;
    if d.is_empty() {
        eprintln!("Off-season, challenger league is empty.");
    }
    Ok(())
}

#[riven_test]
async fn champion_mastery_v4() -> Result<(), String> {
    let summoner = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "LugnutsK");
    let summoner = summoner
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "'LugnutsK' not found!".to_owned())?;
    let masteries = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(ROUTE, &summoner.puuid);
    let masteries = masteries.await.map_err(|e| e.to_string())?;
    rassert!(74 <= masteries.len());
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

#[riven_test]
async fn status() -> Result<(), String> {
    let p = riot_api().lol_status_v4().get_platform_data(ROUTE);
    let status = p.await.map_err(|e| e.to_string())?;
    println!("{:?}", status);
    Ok(())
}

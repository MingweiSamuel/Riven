mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::JP1;

static MATCHES: &[&str] = &[
    // // New field `ParticipantChallenges` `twoWardsOneSweeperCount` (removed 2025-05-26).
    // "JP1_397348569",
    // // New fields: `match-v5.ParticipantDto.playerAugment[1234],playerSubteamId,subteamPlacement`
    // "JP1_400700181",
    // New field: `match-v5.ParticipantDto.placement`
    "JP1_405073638",
    // New ARENA 2v2v2v2 game mode, broken `subteamPlacement`
    "KR_6604607115",
    // New field: `match-v5.ParticipantDto.missions`
    "JP1_417935351",
    // New field: `match-v5.ParticipantDto.riotIdGameName`
    "JP1_419115017",
    // New field: `match-v5.ParticipantChallenges`: `InfernalScalePickup`, `fistBumpParticipation`
    "JP1_447303353",
    // New fields for swarm: `match-v5.ChallengesDto`: `SWARM_DefeatAatrox`, `SWARM_DefeatBriar`
    "JP1_455137806",
    // Swarm solo/duo/trio.
    "JP1_456819992",
    "JP1_457311633",
    "JP1_457196409",
    // `feats` https://github.com/RiotGames/developer-relations/issues/1052
    "JP1_497158018",
];

/// summoner_v4().get_by_summoner_name(...) normally returns an option.
/// If we use `get` (instead of `get_optional`) make sure it errors.
#[riven_test]
async fn get_nonoptional_invalid() -> Result<(), String> {
    let path_string = format!(
        "/lol/summoner/v4/summoners/by-name/{}",
        "SUMMONER THAT DOES NOT EXIST"
    );
    let request = riot_api().request(reqwest::Method::GET, ROUTE.into(), &path_string);
    let p = riot_api().execute_val::<riven::models::summoner_v4::Summoner>(
        "summoner-v4.getBySummonerName",
        ROUTE.into(),
        request,
    );
    let r = p.await;
    rassert!(r.is_err());
    Ok(())
}

/// Check invalid code, make sure 403 is handled as expected.
#[riven_test]
async fn tournament_forbidden() -> Result<(), String> {
    let p = riot_api()
        .tournament_v5()
        .get_tournament_code(ROUTE.to_regional(), "INVALID_CODE");
    let r = p.await;
    rassert!(r.is_err());
    rassert_eq!(
        Some(reqwest::StatusCode::FORBIDDEN),
        r.unwrap_err().status_code()
    );
    Ok(())
}

// Disabled: Caihonbbt no longer ranked.
// /// tft-league-v1.getLeagueEntriesForSummoner
// /// https://github.com/MingweiSamuel/Riven/issues/25
// #[riven_test]
// async fn tft_league_getleagueentriesforsummoner() -> Result<(), String> {
//     let sp = riot_api().summoner_v4().get_by_summoner_name(ROUTE, "Caihonbbt");
//     let sr = sp.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get \"Caihonbbt\"".to_owned())?;
//     let lp = riot_api().tft_league_v1().get_league_entries_for_summoner(ROUTE, &sr.id);
//     let lr = lp.await.map_err(|e| e.to_string())?;
//     rassert!(!lr.is_empty());
//     Ok(())
// }

/// tft-league-v1.getTopRatedLadder
/// https://github.com/MingweiSamuel/Riven/issues/24
#[riven_test]
async fn tft_league_gettopratedladder() -> Result<(), String> {
    let lp = riot_api()
        .tft_league_v1()
        .get_top_rated_ladder(ROUTE, QueueType::RANKED_TFT_TURBO);
    let lr = lp.await.map_err(|e| e.to_string())?;
    rassert!(!lr.is_empty());
    Ok(())
}

/// ASIA regional tests
#[riven_test]
async fn league_v4_match_v5_latest_combo_test() -> Result<(), String> {
    league_v4_match_v5_latest_combo(ROUTE).await
}

#[riven_test]
async fn match_v5_get_test() -> Result<(), String> {
    match_v5_get(ROUTE.to_regional(), MATCHES).await
}

#[riven_test]
async fn match_v5_get_timeline_test() -> Result<(), String> {
    match_v5_get_timeline(ROUTE.to_regional(), MATCHES).await
}

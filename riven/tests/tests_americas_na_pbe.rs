mod testutils;
use riven::consts::*;
use riven::models::tournament_stub_v5::*;
use testutils::*;

const REGION: RegionalRoute = RegionalRoute::AMERICAS;
const PLATFORM: PlatformRoute = PlatformRoute::NA1;
const PLATFORM_PBE: PlatformRoute = PlatformRoute::PBE1;

static MATCHES: &[&str] = &[
    "LA1_1568057368",
    // https://github.com/RiotGames/developer-relations/issues/939#issuecomment-2164119529
    "NA1_5018382378",
    "NA1_5018422066",
    // https://github.com/MingweiSamuel/Riven/issues/84
    "NA1_5245486208",
];

static ACCOUNTS: &[(&str, &str)] = &[
    ("Lug nuts K", "000"),
    // https://github.com/MingweiSamuel/Riven/issues/96
    ("WHITEWOLFF", "270"),
    ("INTIK", "270"),
];

/// Account-v1
#[riven_test]
async fn account_v1_getbyriotid_getbypuuid_getactiveregion() -> Result<(), String> {
    let game_name = "Lug nuts K";
    let tag_line = "000";
    // Game name is case and whitespace insensitive.
    // But tag cannot have spaces. (Is it case sensitive?).
    let account_by_tag = riot_api()
        .account_v1()
        .get_by_riot_id(REGION, game_name, tag_line)
        .await
        .map_err(|e| {
            format!(
                "Failed to get account {}#{} by riot ID: {}",
                game_name, tag_line, e
            )
        })?
        .ok_or("Riot account not found!".to_owned())?;

    let account_by_puuid = riot_api()
        .account_v1()
        .get_by_puuid(REGION, &account_by_tag.puuid)
        .await
        .map_err(|e| {
            format!(
                "Failed to get account {}#{} by PUUID: {}",
                game_name, tag_line, e
            )
        })?;

    assert_eq!(account_by_tag.puuid, account_by_puuid.puuid);

    let active_region = riot_api()
        .account_v1()
        .get_active_region(REGION, "tft", &account_by_tag.puuid)
        .await
        .map_err(|e| {
            format!(
                "Failed to get active region for {}#{}: {}",
                game_name, tag_line, e
            )
        })?;

    assert_eq!(active_region.puuid, account_by_tag.puuid);
    assert_eq!(active_region.game, "tft");
    assert_eq!(active_region.region, "na1");

    Ok(())
}

#[riven_test]
async fn account_v1_getbyriotid_none() -> Result<(), String> {
    let p = riot_api()
        .account_v1()
        .get_by_riot_id(REGION, "this account does not exist", "NA1");
    rassert!(p.await.map_err(|e| e.to_string())?.is_none());
    Ok(())
}

/// Tournament stub test.
#[riven_test]
async fn tournamentstub() -> Result<(), String> {
    let ts = riot_api().tournament_stub_v5();
    let provider_id = ts
        .register_provider_data(
            REGION,
            &ProviderRegistrationParametersV5 {
                region: PLATFORM.to_tournament_region().unwrap(),
                url: "https://github.com/MingweiSamuel/Riven".to_owned(),
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    println!("provider_id: {}", provider_id);

    let tournament_id = ts
        .register_tournament(
            REGION,
            &TournamentRegistrationParametersV5 {
                name: Some("Riven Tourney :)".to_owned()),
                provider_id,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    println!("tournament_id: {}", tournament_id);

    let codes_result = ts
        .create_tournament_code(
            REGION,
            &TournamentCodeParametersV5 {
                map_type: "SUMMONERS_RIFT".to_owned(),
                metadata: Some("eW91IGZvdW5kIHRoZSBzZWNyZXQgbWVzc2FnZQ==".to_owned()),
                pick_type: "TOURNAMENT_DRAFT".to_owned(),
                spectator_type: "ALL".to_owned(),
                team_size: 5,
                allowed_participants: None,
                enough_players: false,
            },
            tournament_id as i64,
            Some(300),
        )
        .await;

    match codes_result {
        Ok(codes) => {
            rassert_eq!(300, codes.len());
            println!("codes: {}", codes.join(", "));
            Ok(())
        }
        Err(mut e) => {
            if let Some(response) = e.take_response() {
                eprintln!("{:?}", response.text().await);
            }
            Err(e.to_string())
        }
    }
}

#[riven_test]
async fn match_v5_get_test() -> Result<(), String> {
    match_v5_get(REGION, MATCHES).await
}

#[riven_test]
async fn match_v5_get_timeline_test() -> Result<(), String> {
    match_v5_get_timeline(REGION, MATCHES).await
}

#[riven_test]
async fn champion_getrotation() -> Result<(), String> {
    let p = riot_api().champion_v3().get_champion_info(PLATFORM);
    let d = p.await.map_err(|e| e.to_string())?;
    let new_len = d.newplayer.len();
    let free_len = d.sr.len();
    rassert!(new_len >= 10, "New len: {}", new_len);
    rassert!(free_len >= 15, "Free len: {}", free_len);
    Ok(())
}

#[riven_test]
async fn league_get_diamond() -> Result<(), String> {
    let p = riot_api().league_v4().get_league_entries(
        PLATFORM,
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
        .get_challenger_league(PLATFORM, QueueType::RANKED_SOLO_5x5);
    let ll = p.await.map_err(|e| e.to_string())?;
    if ll.entries.is_empty() {
        eprintln!("Off-season, challenger league is empty.");
    }
    Ok(())
}

#[riven_test]
async fn leagueexp_get_challenger() -> Result<(), String> {
    let p = riot_api().league_exp_v4().get_league_entries(
        PLATFORM,
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
    let account = riot_api()
        .account_v1()
        .get_by_riot_id(REGION, "LugnutsK", "000");
    let account = account
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "'LugnutsK#000' not found!".to_owned())?;
    let masteries = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(PLATFORM, &account.puuid);
    let masteries = masteries.await.map_err(|e| e.to_string())?;
    rassert!(74 <= masteries.len());
    Ok(())
}

#[riven_test]
async fn championmasteryv4_getall_iamchanese123() -> Result<(), String> {
    let summoner =
        riot_api()
            .account_v1()
            .get_by_riot_id(PLATFORM.to_regional(), "iamchanese123", "NA1");
    let summoner = summoner
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "'iamchanese123#NA1' not found!".to_owned())?;
    let masteries = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(PLATFORM, &summoner.puuid);
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
    let p = riot_api().clash_v1().get_tournaments(PLATFORM);
    let tours = p.await.map_err(|e| e.to_string())?;
    if let Some(tour0) = tours.first() {
        let p = riot_api()
            .clash_v1()
            .get_tournament_by_id(PLATFORM, tour0.id);
        let tour1 = p.await.map_err(|e| e.to_string())?;
        assert_eq!(Some(tour0.id), tour1.map(|t| t.id));
    }
    Ok(())
}

#[riven_test]
async fn clash_get_team_by_id_invalid() -> Result<(), String> {
    let p = riot_api()
        .clash_v1()
        .get_team_by_id(PLATFORM, "00000000-0000-0000-0000-000000000000");
    let team = p.await.map_err(|e| e.to_string())?;
    assert!(team.is_none());
    Ok(())
}

// STATUS
#[riven_test]
async fn status() -> Result<(), String> {
    let p = riot_api().lol_status_v4().get_platform_data(PLATFORM);
    let status = p.await.map_err(|e| e.to_string())?;
    let _ = status;
    Ok(())
}

#[riven_test]
async fn account_summoner_by_puuid() -> Result<(), String> {
    let futures = ACCOUNTS.iter().map(|&(game_name, tag_line)| async move {
        let account = riot_api()
            .account_v1()
            .get_by_riot_id(REGION, game_name, tag_line)
            .await
            .map_err(|e| {
                format!(
                    "Failed to get account {}#{} by riot ID: {}",
                    game_name, tag_line, e
                )
            })?
            .ok_or_else(|| format!("Account {}#{} not found!", game_name, tag_line))?;

        let summoner = riot_api()
            .summoner_v4()
            .get_by_puuid(PLATFORM, &account.puuid)
            .await
            .map_err(|e| {
                format!(
                    "Failed to get summoner by PUUID for {}#{}: {}",
                    game_name, tag_line, e
                )
            })?;

        if let Some(summoner) = summoner {
            assert_eq!(
                account.puuid, summoner.puuid,
                "PUUID mismatch for {}#{}",
                game_name, tag_line
            );
        } else {
            eprintln!("Summoner not found (404) for {}#{}", game_name, tag_line);
        }

        Ok(())
    });

    join_all_future_errs(futures).await
}

#[riven_test]
async fn match_v5_get_replay_canttype1998() -> Result<(), String> {
    match_v5_get_replay(REGION, "cant type", "1998").await
}

/// Test 0 replays.
#[riven_test]
async fn match_v5_get_replay_whitewolff270() -> Result<(), String> {
    match_v5_get_replay(REGION, "WHITEWOLFF", "270").await
}

/// Get matches from PBE.
#[riven_test]
async fn league_v4_match_v5_latest_combo_test() -> Result<(), String> {
    league_v4_match_v5_latest_combo(PLATFORM_PBE).await
}

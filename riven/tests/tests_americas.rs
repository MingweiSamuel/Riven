mod testutils;
use riven::consts::*;
use riven::models::tournament_stub_v5::*;
use testutils::*;

const ROUTE: RegionalRoute = RegionalRoute::AMERICAS;

static MATCHES: &[&str] = &[
    "NA1_4265913704", // `match-v5.ParticipantDto.challenges.mejaisFullStackInTime`
    "NA1_4924008147",
];

/// Account-v1
#[riven_test]
async fn account_v1_getbyriotid_getbypuuid() -> Result<(), String> {
    // Game name is case and whitespace insensitive.
    // But tag cannot have spaces. (Is it case sensitive?).
    let account_tag = riot_api()
        .account_v1()
        .get_by_riot_id(ROUTE, "Lug nuts K", "000")
        .await
        .map_err(|e| format!("Failed to get account by riot ID: {}", e))?
        .ok_or("Riot account not found!".to_owned())?;

    let account_puuid = riot_api()
        .account_v1()
        .get_by_puuid(ROUTE, &account_tag.puuid)
        .await
        .map_err(|e| format!("Failed to get account by PUUID: {}", e))?;

    let _ = account_puuid;

    Ok(())
}

/// Tournament stub test.
#[riven_test]
async fn tournamentstub() -> Result<(), String> {
    let ts = riot_api().tournament_stub_v5();
    let provider_id = ts
        .register_provider_data(
            ROUTE,
            &ProviderRegistrationParametersV5 {
                region: PlatformRoute::NA1.as_region_str().to_owned(),
                url: "https://github.com/MingweiSamuel/Riven".to_owned(),
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    println!("provider_id: {}", provider_id);

    let tournament_id = ts
        .register_tournament(
            ROUTE,
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
            ROUTE,
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
    match_v5_get(ROUTE, MATCHES).await
}

#[riven_test]
async fn match_v5_get_timeline_test() -> Result<(), String> {
    match_v5_get_timeline(ROUTE, MATCHES).await
}

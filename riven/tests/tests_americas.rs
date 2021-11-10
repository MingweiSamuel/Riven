#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;
use riven::models::tournament_stub_v4::*;

const ROUTE: RegionalRoute = RegionalRoute::AMERICAS;

static MATCHES: [&str; 5] = [
    "NA1_3923487226",
    "NA1_4049206905",
    "NA1_4052515784",
    "NA1_4062578191",
    "NA1_4097036960",
];

async_tests!{
    my_runner {
        // Champion Mastery tests.
        tournamentstub: async {
            let tsv4 = RIOT_API.tournament_stub_v4();
            let provider_id = tsv4.register_provider_data(ROUTE, &ProviderRegistrationParameters {
                region: PlatformRoute::NA1.as_region_str().to_owned(),
                url: "https://github.com/MingweiSamuel/Riven".to_owned(),
            })
                .await
                .map_err(|e| e.to_string())?;

            println!("provider_id: {}", provider_id);

            let tournament_id = tsv4.register_tournament(ROUTE, &TournamentRegistrationParameters {
                name: Some("Riven Tourney :)".to_owned()),
                provider_id,
            })
                .await
                .map_err(|e| e.to_string())?;

            println!("tournament_id: {}", tournament_id);

            let codes_result = tsv4.create_tournament_code(ROUTE, &TournamentCodeParameters {
                map_type: "SUMMONERS_RIFT".to_owned(),
                metadata: Some("eW91IGZvdW5kIHRoZSBzZWNyZXQgbWVzc2FnZQ==".to_owned()),
                pick_type: "TOURNAMENT_DRAFT".to_owned(),
                spectator_type: "ALL".to_owned(),
                team_size: 5,
                allowed_summoner_ids: None,
            }, tournament_id as i64, Some(300))
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
        },

        match_v5_get: async {
            for matche in MATCHES {
                let p = RIOT_API.match_v5().get_match(ROUTE, matche);
                let m = p.await.map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?.ok_or(format!("Match {} not found.", matche))?;
                rassert_eq!(matche, m.metadata.match_id, "Bad match id? Sent {}, received {}.", matche, m.metadata.match_id);
                rassert!(!m.metadata.participants.is_empty(), "Match should have participants.");
                rassert!(!m.info.teams.is_empty(), "Match should have teams.");
            }
            Ok(())
        },
        match_v5_get_timeline: async {
            for matche in MATCHES {
                let p = RIOT_API.match_v5().get_timeline(ROUTE, matche);
                let m = p.await.map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?.ok_or(format!("Match {} not found.", matche))?;
                rassert_eq!(matche, m.metadata.match_id, "Bad match id? Sent {}, received {}.", matche, m.metadata.match_id);
                rassert!(!m.metadata.participants.is_empty(), "Match should have participants.");
                if let Some(game_id) = m.info.game_id {
                    rassert_eq!(matche[(matche.find('_').unwrap() + 1)..], format!("{}", game_id), "Match number ID should match.");
                }
                rassert!(!m.info.frames.is_empty(), "Match timleine should have frames.");
            }
            Ok(())
        },
    }
}

#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;
// use riven::models::tournament_stub_v4::*;

const ROUTE: RegionalRoute = RegionalRoute::AMERICAS;

static MATCHES: &[&str] = &[
    "NA1_4097036960",
    // New games with `match-v5.ParticipantDto.challenges` field.
    "NA1_4209556127",
    "NA1_4212715433",
    "NA1_4265913704", // `match-v5.ParticipantDto.challenges.mejaisFullStackInTime`
];

async_tests! {
    my_runner {
        // TODO FAILING since 2022/11/28 https://github.com/MingweiSamuel/Riven/actions/runs/3571320200/jobs/6003088646
        // // Champion Mastery tests.
        // tournamentstub: async {
        //     let tsv4 = RIOT_API.tournament_stub_v4();
        //     let provider_id = tsv4.register_provider_data(ROUTE, &ProviderRegistrationParameters {
        //         region: PlatformRoute::NA1.as_region_str().to_owned(),
        //         url: "https://github.com/MingweiSamuel/Riven".to_owned(),
        //     })
        //         .await
        //         .map_err(|e| e.to_string())?;

        //     println!("provider_id: {}", provider_id);

        //     let tournament_id = tsv4.register_tournament(ROUTE, &TournamentRegistrationParameters {
        //         name: Some("Riven Tourney :)".to_owned()),
        //         provider_id,
        //     })
        //         .await
        //         .map_err(|e| e.to_string())?;

        //     println!("tournament_id: {}", tournament_id);

        //     let codes_result = tsv4.create_tournament_code(ROUTE, &TournamentCodeParameters {
        //         map_type: "SUMMONERS_RIFT".to_owned(),
        //         metadata: Some("eW91IGZvdW5kIHRoZSBzZWNyZXQgbWVzc2FnZQ==".to_owned()),
        //         pick_type: "TOURNAMENT_DRAFT".to_owned(),
        //         spectator_type: "ALL".to_owned(),
        //         team_size: 5,
        //         allowed_summoner_ids: None,
        //     }, tournament_id as i64, Some(300))
        //         .await;

        //     match codes_result {
        //         Ok(codes) => {
        //             rassert_eq!(300, codes.len());
        //             println!("codes: {}", codes.join(", "));
        //             Ok(())
        //         }
        //         Err(mut e) => {
        //             if let Some(response) = e.take_response() {
        //                 eprintln!("{:?}", response.text().await);
        //             }
        //             Err(e.to_string())
        //         }
        //     }
        // },

        match_v5_get: async {
            match_v5_get(ROUTE, MATCHES).await
        },
        match_v5_get_timeline: async {
            match_v5_get_timeline(ROUTE, MATCHES).await
        },
    }
}

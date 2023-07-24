#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;
use riven::models::summoner_v4::*;
// use riven::models::tournament_stub_v4::*;

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

static MATCHES: &[&str] = &[
    "NA1_3923487226",
    "NA1_4049206905",
    "NA1_4052515784",
    "NA1_4062578191",
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
            match_v5_get(ROUTE.to_regional(), MATCHES).await
        },
        match_v5_get_timeline: async {
            match_v5_get_timeline(ROUTE.to_regional(), MATCHES).await
        },

        // Summoner tests.
        summoner_double: async {
            let l1p = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "lug nuts k");
            let l2p = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "lugnuts k");
            let l1 = l1p.await.map_err(|e| e.to_string())?.ok_or_else(|| "'lug nuts k' not found!".to_owned())?;
            let l2 = l2p.await.map_err(|e| e.to_string())?.ok_or_else(|| "'lugnuts k' not found!".to_owned())?;
            validate_summoners(l1, l2)?;
            Ok(())
        },
        champion_getrotation: async {
            let p = RIOT_API.champion_v3().get_champion_info(ROUTE);
            let d = p.await.map_err(|e| e.to_string())?;
            let new_len = d.free_champion_ids_for_new_players.len();
            let free_len = d.free_champion_ids.len();
            let level = d.max_new_player_level;
            rassert!(new_len  >= 10, "New len: {}", new_len);
            rassert!(free_len >= 15, "Free len: {}", free_len);
            rassert_eq!(10, level, "New player level: {}", level);
            Ok(())
        },
        leagueexp_get: async {
            let p = RIOT_API.league_exp_v4().get_league_entries(ROUTE, QueueType::RANKED_SOLO_5x5, Tier::CHALLENGER, Division::I, None);
            let d = p.await.map_err(|e| e.to_string())?;
            if d.is_empty() {
                eprintln!("Off-season, challenger league is empty.");
            }
            Ok(())
        },
        champion_mastery_v4: async {
            let summoner = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "LugnutsK");
            let summoner = summoner.await.map_err(|e| e.to_string())?.ok_or_else(|| "'LugnutsK' not found!".to_owned())?;
            let masteries = RIOT_API.champion_mastery_v4().get_all_champion_masteries(ROUTE, &summoner.id);
            let masteries = masteries.await.map_err(|e| e.to_string())?;
            rassert!(74 <= masteries.len());
            Ok(())
        },

        // TO TEST THIS BUG: https://github.com/RiotGames/developer-relations/issues/572.
        // https://lolchess.gg/leaderboards?mode=doubleup&region=na
        // summoner must have double-up rank.
        league_getforsummoner_tftbug: async {
            // TODO(mingwei): get summoner from leaderboard to avoid updating this all the time.
            const SUMMONER_NAME: &str = "jessixa";
            let summoner_fut = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, SUMMONER_NAME);
            let summoner = summoner_fut.await.map_err(|e| e.to_string())?.ok_or_else(|| format!("Failed to get \"{}\"", SUMMONER_NAME))?;
            let league_fut = RIOT_API.league_v4().get_league_entries_for_summoner(ROUTE, &summoner.id);
            let leagues = league_fut.await.map_err(|e| e.to_string())?;
            let tft_league = leagues.iter().find(|league| QueueType::RANKED_TFT_DOUBLE_UP == league.queue_type);
            rassert!(tft_league.is_some());
            Ok(())
        },

        // TODO: MATCH-V4 REMOVED.
        // matchlist_get: async {
        //     let sp = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "haha yes");
        //     let s = sp.await.map_err(|e| e.to_string())?.ok_or("Failed to get \"haha yes\"".to_owned())?;
        //     let mp = RIOT_API.match_v4().get_matchlist(ROUTE, &s.account_id, None, Some(2500), None, None, Some(2600), None, None);
        //     let m = mp.await.map_err(|e| e.to_string())?.ok_or("Failed to get matchlist".to_owned())?;
        //     rassert!(m.matches.len() > 0, "Matchlist should not be empty");
        //     Ok(())
        // },
        // matchlist_get2: async {
        //     let sp = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "haha yes");
        //     let s = sp.await.map_err(|e| e.to_string())?.ok_or("Failed to get \"haha yes\"".to_owned())?;
        //     let mp = RIOT_API.match_v4().get_matchlist(ROUTE, &s.account_id, None, None, Some(&[ Champion::SION, Champion::SIVIR, Champion::CASSIOPEIA ]), None, None, None, None);
        //     let m = mp.await.map_err(|e| e.to_string())?.ok_or("Failed to get matchlist".to_owned())?;
        //     rassert!(m.matches.len() > 0, "Matchlist should not be empty");
        //     Ok(())
        // },

        // match_get: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 3190191338);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Match not found.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_bots: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 3251803350);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Match not found.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_odyssey: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 2881976826);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Match not found.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_aram: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 2961635718);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get match.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_aram2: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 3596184782);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Match not found.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_urf900: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 2963663381);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get match.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_tutorial1: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 3432145099);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get match.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_tutorial2: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 3432116214);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get match.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },
        // match_get_tutorial3: async {
        //     let p = RIOT_API.match_v4().get_match(ROUTE, 3432156790);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get match.".to_owned())?;
        //     rassert!(!m.participants.is_empty(), "Match should have participants.");
        //     Ok(())
        // },

        // match_gettimeline: async {
        //     let p = RIOT_API.match_v4().get_match_timeline(ROUTE, 3190191338);
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Match timeline not found.".to_owned())?;
        //     rassert!(!m.frames.is_empty(), "Match timeline should have frames.");
        //     Ok(())
        // },

        // Commented out, requires special API key.
        // // LOR
        // lor_ranked_get_leaderboards: async {
        //     let future = RIOT_API.lor_ranked_v1().get_leaderboards(Region::AMERICAS);
        //     let _leaderboard = future.await.map_err(|e| e.to_string())?;
        //     Ok(())
        // },
        // CLASH
        clash_get_tournaments: async {
            let p = RIOT_API.clash_v1().get_tournaments(ROUTE);
            let tours = p.await.map_err(|e| e.to_string())?;
            if let Some(tour0) = tours.first() {
                let p = RIOT_API.clash_v1().get_tournament_by_id(ROUTE, tour0.id);
                let tour1 = p.await.map_err(|e| e.to_string())?;
                assert_eq!(Some(tour0.id), tour1.map(|t| t.id));
            }
            Ok(())
        },
        clash_get_team_by_id: async {
            let p = RIOT_API.clash_v1().get_team_by_id(ROUTE, "00000000-0000-0000-0000-000000000000");
            let team = p.await.map_err(|e| e.to_string())?;
            assert!(team.is_none());
            Ok(())
        },

        status: async {
            let p = RIOT_API.lol_status_v4().get_platform_data(ROUTE);
            let status = p.await.map_err(|e| e.to_string())?;
            println!("{:?}", status);
            Ok(())
        },

        summoner_history: async {
            let summoner = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "Bang Fangirl");
            let summoner = summoner.await.map_err(|e| e.to_string())?.ok_or_else(|| "'Bang Fangirl' not found!".to_owned())?;

            let ids = RIOT_API
                .match_v5()
                .get_match_ids_by_puuid(
                    ROUTE.to_regional(),
                    &summoner.puuid,
                    Some(10),
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await
                .map_err(|e| e.to_string())?;

            for id in ids {
                let _match_data = RIOT_API
                    .match_v5()
                    .get_match(ROUTE.to_regional(), id.as_str())
                    .await
                    .map_err(|e| e.to_string())?;
            }

            Ok(())
        },
    }
}

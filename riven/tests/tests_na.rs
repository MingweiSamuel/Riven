#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;
use riven::models::summoner_v4::*;

fn validate_summoners(s1: Summoner, s2: Summoner) -> Result<(), String> {
    rassert_eq!(s1.name, s2.name, "Names didn't match {}.", "");
    rassert_eq!(s1.id, s2.id, "SummonerId didn't match {}.", "");
    rassert_eq!(s1.account_id, s2.account_id, "AccountId didn't match {}.", "");
    Ok(())
}

const ROUTE: PlatformRoute = PlatformRoute::NA1;

async_tests!{
    my_runner {
        // Summoner tests.
        summoner_double: async {
            let l1p = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "lug nuts k");
            let l2p = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "lugnuts k");
            let l1 = l1p.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get l1".to_owned())?;
            let l2 = l2p.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get l2".to_owned())?;
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
            rassert!(!d.is_empty(), "Challenger shouldn't be empty.");
            Ok(())
        },

        // TO TEST THIS BUG: https://github.com/RiotGames/developer-relations/issues/572.
        league_getforsummoner_tftbug: async {
            const SUMMONER_NAME: &'static str = "LE ANTIFRAGILE";
            let summoner_fut = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, SUMMONER_NAME);
            let summoner = summoner_fut.await.map_err(|e| e.to_string())?.ok_or_else(|| format!("Failed to get \"{}\"", SUMMONER_NAME))?;
            let league_fut = RIOT_API.league_v4().get_league_entries_for_summoner(ROUTE, &*summoner.id);
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
    }
}

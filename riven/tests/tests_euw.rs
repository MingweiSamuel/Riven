#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;

const ROUTE: PlatformRoute = PlatformRoute::EUW1;

async_tests!{
    my_runner {
        // Champion Mastery tests.
        championmastery_getscore_ma5tery: async {
            let sum = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "ma5tery");
            let sum = sum.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get summoner".to_owned())?;

            let p = RIOT_API.champion_mastery_v4().get_champion_mastery_score(ROUTE, &*sum.id);
            let s = p.await.map_err(|e| e.to_string())?;
            rassert!((969..=1000).contains(&s), "Unexpected ma5tery score: {}.", s);
            Ok(())
        },
        championmastery_getall_ma5tery: async {
            let sum = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "ma5tery");
            let sum = sum.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get summoner".to_owned())?;

            let p = RIOT_API.champion_mastery_v4().get_all_champion_masteries(ROUTE, &*sum.id);
            let s = p.await.map_err(|e| e.to_string())?;
            rassert!(s.len() >= 142, "Expected masteries: {}.", s.len());
            Ok(())
        },

        // TODO: https://github.com/RiotGames/developer-relations/issues/602
        // spectator_combo: async {
        //     let featured_p = RIOT_API.spectator_v4().get_featured_games(ROUTE);
        //     let featured = featured_p.await.map_err(|e| e.to_string())?;

        //     rassert!(!featured.game_list.is_empty());

        //     let summoner_name = &featured.game_list[0].participants[0].summoner_name;
        //     let summoner_p = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, summoner_name);
        //     let summoner = summoner_p.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get summoner".to_owned())?;

        //     let livegame_p = RIOT_API.spectator_v4().get_current_game_info_by_summoner(ROUTE, &summoner.id);
        //     let livegame_o = livegame_p.await.map_err(|e| e.to_string())?;
        //     if let Some(livegame) = livegame_o {
        //         let participant_match = livegame.participants.iter().find(|p| p.summoner_name == *summoner_name);
        //         rassert!(participant_match.is_some(), "Failed to find summoner in match: {}.", summoner_name);
        //     }
        //     Ok(())
        // },

        // // TFT tests.
        // tftleaguev1_getchallengerleague: async {
        //     let p = RIOT_API.tft_league_v1().get_challenger_league(Region::EUW);
        //     let l = p.await.map_err(|e| e.to_string())?;
        //     rassert!(l.entries.len() > 10, "Expected a few challenger players, got: {}.", l.entries.len());
        //     Ok(())
        // },
        // tftmatchv1_getmatch: async {
        //     let p = RIOT_API.tft_match_v1().get_match(Region::EUROPE, "EUW1_4568680990");
        //     let _m = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get TFT match.".to_owned())?;
        //     Ok(())
        // },
        // tftsummonerv1_getbyname: async {
        //     let p = RIOT_API.tft_summoner_v1().get_by_summoner_name(Region::EUW, "相当猥琐");
        //     let _s = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get TFT summoner.".to_owned())?;
        //     Ok(())
        // },
        // tftsummonerv1_getbyname_none: async {
        //     let p = RIOT_API.tft_summoner_v1().get_by_summoner_name(Region::EUW, "this summoner does not exist");
        //     rassert!(p.await.map_err(|e| e.to_string())?.is_none());
        //     Ok(())
        // },
    }
}

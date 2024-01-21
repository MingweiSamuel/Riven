#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use colored::*;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::EUW1;

static TFT_MATCHES: &[&str] = &[
    "EUW1_6307427444", // https://github.com/MingweiSamuel/Riven/issues/50
    "EUW1_6307262798",
];

async_tests! {
    my_runner {
        tftmatchv1_get_list: async {
            tft_match_v1_get(ROUTE.to_regional(), TFT_MATCHES).await
        },

        // // Don't have acecess to tft-status-v1.
        // tftstatusv1_getplatformdata: async {
        //     let p = RIOT_API.tft_status_v1().get_platform_data(ROUTE);
        //     let _s = p.await.map_err(|e| e.to_string())?;
        //     Ok(())
        // },
        tftleaguev1_gettopratedladder: async {
            let p = RIOT_API.tft_league_v1().get_top_rated_ladder(ROUTE, QueueType::RANKED_TFT_TURBO);
            let l = p.await.map_err(|e| e.to_string())?;
            rassert!(l.len() > 10, "Expected a few ranked players, got: {}.", l.len());
            Ok(())
        },
        tftmatchv1_getmatch: async {
            let p = RIOT_API.tft_match_v1().get_match(ROUTE.to_regional(), "EUW1_6455483163");
            let _m = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get TFT match.".to_owned())?;
            Ok(())
        },
        tftsummonerv1_getbyname: async {
            let p = RIOT_API.tft_summoner_v1().get_by_summoner_name(ROUTE, "相当猥琐");
            let _s = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get TFT summoner.".to_owned())?;
            Ok(())
        },
        tftsummonerv1_getbyname_none: async {
            let p = RIOT_API.tft_summoner_v1().get_by_summoner_name(ROUTE, "this summoner does not exist");
            rassert!(p.await.map_err(|e| e.to_string())?.is_none());
            Ok(())
        },
        // Get top rated player, get some of their matches.
        tft_combo: async {
            let top_players = RIOT_API.tft_league_v1().get_top_rated_ladder(ROUTE, QueueType::RANKED_TFT_TURBO);
            let top_players = top_players.await.map_err(|e| e.to_string())?;
            rassert!(!top_players.is_empty());
            let top_player_entry = &top_players[0];
            let top_player = RIOT_API.tft_summoner_v1().get_by_summoner_id(ROUTE, &top_player_entry.summoner_id);
            let top_player = top_player.await.map_err(|e| e.to_string())?;
            println!("Top player is {} with `puuid` {}.", top_player.name, top_player.puuid);
            let match_ids = RIOT_API.tft_match_v1().get_match_ids_by_puuid(
                ROUTE.to_regional(), &top_player.puuid, Some(10), None, None, None);
            let match_ids = match_ids.await.map_err(|e| e.to_string())?;
            tft_match_v1_get(ROUTE.to_regional(), &*match_ids).await?;
            Ok(())
        },
    }
}

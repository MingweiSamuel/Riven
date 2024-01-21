#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use colored::*;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::RU;

async_tests! {
    my_runner {
        summoner_leagues: async {
            let sum = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "d3atomiz3d");
            let sum = sum.await
                .map_err(|e| format!("Error getting summoner: {}", e))?
                .ok_or_else(|| "Failed to find summoner".to_owned())?;

            let p = RIOT_API.league_v4().get_league_entries_for_summoner(ROUTE, &sum.id);
            let s = p.await.map_err(|e| format!("Error getting league entries: {}", e))?;
            let _ = s;
            Ok(())
        },
    }
}

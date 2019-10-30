#![feature(custom_test_frameworks)]
#![test_runner(my_runner)]

mod async_tests;
mod ids;

use colored::*;
use lazy_static::lazy_static;
use tokio::runtime::current_thread::Runtime;

use riven::RiotApi;
use riven::consts::*;


lazy_static! {
    static ref RIOT_API: RiotApi = {
        let api_key = std::fs::read_to_string("apikey.txt").unwrap();
        RiotApi::with_key(api_key.trim())
    };
}

async_tests!{
    my_runner {
        // Champion Mastery tests.
        championmastery_getscore_ma5tery: async {
            let p = RIOT_API.champion_mastery_v4().get_champion_mastery_score(Region::EUW, ids::SUMMONER_ID_MA5TERY);
            let s = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get ma5tery".to_owned())?;
            rassert!(969 <= s && s <= 1000, "Unexpected ma5tery score: {}.", s);
            Ok(())
        },
        championmastery_getall_ma5tery: async {
            let p = RIOT_API.champion_mastery_v4().get_all_champion_masteries(Region::EUW, ids::SUMMONER_ID_MA5TERY);
            let s = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get ma5tery".to_owned())?;
            rassert!(s.len() >= 142, "Expected masteries: {}.", s.len());
            Ok(())
        },
    }
}

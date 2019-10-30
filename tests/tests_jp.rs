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
        // Summoner tests.
        summoner_get_kanjikana: async {
            let p = RIOT_API.summoner_v4().get_by_summoner_name(Region::JP, "私の 頭が かたい");
            let s = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get myheadhard".to_owned())?;
            rassert_eq!("私の頭がかたい", s.name);
            Ok(())
        },
    }
}

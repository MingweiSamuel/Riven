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
        league_summoner_bulk_test: async {
            let p = RIOT_API.league_v4().get_challenger_league(Region::TR, QueueType::RANKED_SOLO_5x5);
            let ll = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get challenger league".to_owned())?;
            // println!("{:#?}", ll);
            // TODO!!!
            Ok(())
        },
    }
}

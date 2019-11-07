#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;

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

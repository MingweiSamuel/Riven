#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
// use testutils::*;

use colored::*;

// use riven::consts::*;

async_tests!{
    my_runner {
        // DISABLED FOR API KEY ACCESS.
        // match_v5_get: async {
        //     let p = RIOT_API.match_v5().get_match(Region::AMERICAS, "NA1_3923487226");
        //     let m = p.await.map_err(|e| e.to_string())?.ok_or("Match not found.".to_owned())?;
        //     rassert_eq!("NA1_3923487226", m.metadata.match_id, "Bad match id? {}", m.metadata.match_id);
        //     rassert!(!m.metadata.participants.is_empty(), "Match should have participants.");
        //     rassert!(!m.info.teams.is_empty(), "Match should have teams.");
        //     Ok(())
        // },
    }
}

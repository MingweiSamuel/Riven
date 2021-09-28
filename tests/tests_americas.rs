#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;

static MATCHES: [&'static str; 3] = [ "NA1_3923487226", "NA1_4049206905", "NA1_4052515784" ];

async_tests!{
    my_runner {
        match_v5_get: async {
            for matche in MATCHES {
                let p = RIOT_API.match_v5().get_match(Region::AMERICAS, matche);
                let m = p.await.map_err(|e| e.to_string())?.ok_or(format!("Match {} not found.", matche))?;
                rassert_eq!(matche, m.metadata.match_id, "Bad match id? Sent {}, received {}.", matche, m.metadata.match_id);
                rassert!(!m.metadata.participants.is_empty(), "Match should have participants.");
                rassert!(!m.info.teams.is_empty(), "Match should have teams.");
            }
            Ok(())
        },
        match_v5_get_timeline: async {
            for matche in MATCHES {
                let p = RIOT_API.match_v5().get_timeline(Region::AMERICAS, matche);
                let m = p.await.map_err(|e| e.to_string())?.ok_or(format!("Match timeline {} not found.", matche))?;
                rassert_eq!(matche, m.metadata.match_id, "Bad match id? Sent {}, received {}.", matche, m.metadata.match_id);
                rassert!(!m.metadata.participants.is_empty(), "Match should have participants.");
                rassert_eq!(matche, format!("NA1_{}", m.info.game_id), "Match number ID should match.");
                rassert!(!m.info.frames.is_empty(), "Match timleine should have frames.");
            }
            Ok(())
        },
    }
}

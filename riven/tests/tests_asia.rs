#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;

const ROUTE: RegionalRoute = RegionalRoute::ASIA;

static MATCHES: [&str; 11] = [
    // Regular game:
    "KR_5495121707",
    // `teamPosition` empty:
    // AFK:
    "JP1_312062554",
    "JP1_326464722",
    "JP1_289504387",
    "JP1_285434511",
    "JP1_307559381",
    "JP1_292569767",
    "JP1_310138781",
    "JP1_300507433",
    "JP1_283568774",
    // `individualPosition` is set but `teamPosition` is empty due to AFK slightly after beginning:
    "JP1_285797147",
];

async_tests!{
    my_runner {
        match_v5_get: async {
            for matche in MATCHES {
                let p = RIOT_API.match_v5().get_match(ROUTE, matche);
                let m = p.await.map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?.ok_or(format!("Match {} not found.", matche))?;
                rassert_eq!(matche, m.metadata.match_id, "Bad match id? Sent {}, received {}.", matche, m.metadata.match_id);
                rassert!(!m.metadata.participants.is_empty(), "Match should have participants.");
                rassert!(!m.info.teams.is_empty(), "Match should have teams.");
            }
            Ok(())
        },
        match_v5_get_timeline: async {
            for matche in MATCHES {
                let p = RIOT_API.match_v5().get_timeline(ROUTE, matche);
                let m = p.await.map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?.ok_or(format!("Match {} not found.", matche))?;
                rassert_eq!(matche, m.metadata.match_id, "Bad match id? Sent {}, received {}.", matche, m.metadata.match_id);
                rassert!(!m.metadata.participants.is_empty(), "Match should have participants.");
                if let Some(game_id) = m.info.game_id {
                    rassert_eq!(matche[(matche.find('_').unwrap() + 1)..], format!("{}", game_id), "Match number ID should match.");
                }
                rassert!(!m.info.frames.is_empty(), "Match timleine should have frames.");
            }
            Ok(())
        },
    }
}

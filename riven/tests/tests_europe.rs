#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;

const ROUTE: RegionalRoute = RegionalRoute::EUROPE;

static MATCHES: &[&str] = &[
    // Illegal big `championId`s. https://github.com/RiotGames/developer-relations/issues/553
    "EUW1_5097684633",
    "EUW1_5097963383",
    "EUW1_5102203800", // https://github.com/MingweiSamuel/Riven/issues/36
    "EUW1_5765650307", // https://gist.github.com/MingweiSamuel/d5f9dc40cc5a80a9255e488f27705c56?permalink_comment_id=4088256#gistcomment-4088256
];

async_tests! {
    my_runner {
        match_v5_get: async {
            match_v5_get(ROUTE, MATCHES).await
        },
        match_v5_get_timeline: async {
            match_v5_get_timeline(ROUTE, MATCHES).await
        },
    }
}

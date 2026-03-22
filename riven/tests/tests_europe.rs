mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: RegionalRoute = RegionalRoute::EUROPE;

// Archived 2023-08-17
// // Illegal big `championId`s. https://github.com/RiotGames/developer-relations/issues/553
// "EUW1_5097684633",
// "EUW1_5097963383",
// "EUW1_5102203800", // https://github.com/MingweiSamuel/Riven/issues/36
// "EUW1_5765650307", // https://gist.github.com/MingweiSamuel/d5f9dc40cc5a80a9255e488f27705c56?permalink_comment_id=4088256#gistcomment-4088256
// "EUW1_6349186754", // https://github.com/MingweiSamuel/Riven/issues/71
// // Added 2023-08-27
// "EUW1_6569580003",
// "EUW1_6569417645",
// "EUW1_6568707352",
// "EUW1_6568635198",
// "EUW1_6568537080",
// //
// "EUW1_6569580003",
// "EUW1_6834713231", // `game_id` is zero.

static MATCHES: &[&str] = &[
    // Removed EUW1_6852390800 (previously: Timeline `OBJECTIVE_BOUNTY_PRESTART`
    // https://github.com/MingweiSamuel/riotapi-schema/issues/45) — match no longer
    // exists in the Riot API ("not found"), causing CI failures due to ephemeral
    // external data.
    // SWIFTPLAY
    "EUW1_7261321891",
    "EUW1_7333077176",
    // https://github.com/RiotGames/developer-relations/issues/939#issuecomment-2164112865
    "RU_490603994",
    // https://github.com/RiotGames/developer-relations/issues/1076
    "EUW1_7400007273",
    "EUW1_7399991093",
    "EUW1_7399939488",
    "EUW1_7399909108",
    "EUW1_7399899238",
    "EUW1_7399886298",
];

#[riven_test]
async fn match_v5_get_test() -> Result<(), String> {
    match_v5_get(ROUTE, MATCHES).await
}

#[riven_test]
async fn match_v5_get_timeline_test() -> Result<(), String> {
    match_v5_get_timeline(ROUTE, MATCHES).await
}

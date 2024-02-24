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

static MATCHES: &[&str] = &[
    // New ARENA 2v2v2v2 game mode
    "EUW1_6511808246", // https://github.com/MingweiSamuel/Camille/issues/99
    // Added 2023-08-27
    "EUW1_6569580003",
    "EUW1_6569417645",
    "EUW1_6568707352",
    "EUW1_6568635198",
    "EUW1_6568537080",
];

#[tokio_shared_rt::test]
async fn match_v5_get_test() -> Result<(), String> {
    match_v5_get(ROUTE, MATCHES).await
}

#[tokio_shared_rt::test]
async fn match_v5_get_timeline_test() -> Result<(), String> {
    match_v5_get_timeline(ROUTE, MATCHES).await
}

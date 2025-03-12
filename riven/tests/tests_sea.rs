mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: RegionalRoute = RegionalRoute::SEA;

static MATCHES: &[&str] = &[
    // https://github.com/MingweiSamuel/Riven/issues/65
    "TH2_24825892",
    // https://github.com/RiotGames/developer-relations/issues/939#issuecomment-2164112865
    "SG2_31726207",
    "TW2_205251003",
];

#[riven_test]
async fn match_v5_get_test() -> Result<(), String> {
    match_v5_get(ROUTE, MATCHES).await
}

#[riven_test]
async fn match_v5_get_timeline_test() -> Result<(), String> {
    match_v5_get_timeline(ROUTE, MATCHES).await
}

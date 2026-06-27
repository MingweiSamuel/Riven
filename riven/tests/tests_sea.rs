mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: RegionalRoute = RegionalRoute::SEA;

static MATCHES: &[&str] = &[
    // https://github.com/RiotGames/developer-relations/issues/1076
    "SG2_72450920",
    "SG2_72421858",
];

#[riven_test]
async fn match_v5_get_test() -> Result<(), String> {
    match_v5_get(ROUTE, MATCHES).await
}

#[riven_test]
async fn match_v5_get_timeline_test() -> Result<(), String> {
    match_v5_get_timeline(ROUTE, MATCHES).await
}

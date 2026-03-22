mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: RegionalRoute = RegionalRoute::SEA;

static MATCHES: &[&str] = &[
    // Removed TH2_24825892 (previously: https://github.com/MingweiSamuel/Riven/issues/65) —
    // match no longer exists in the Riot API ("not found"), causing CI failures due to
    // reliance on ephemeral external data that becomes invalid over time.
    // https://github.com/RiotGames/developer-relations/issues/939#issuecomment-2164112865
    "SG2_31726207",
    "TW2_205251003",
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

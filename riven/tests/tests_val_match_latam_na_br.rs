mod testutils;
use riven::consts::*;
use testutils::{riven_test, val_content_ranked, val_match_v1_get, val_match_v1_latest};

const ROUTE: ValPlatformRoute = ValPlatformRoute::NA;

static MATCHES: &[&str] = &[
    // // Aborted game (removed 2025-05-26)
    // "4c196b0a-b396-46ae-a937-b4c52ee83f9a",
    // https://github.com/MingweiSamuel/Riven/issues/92
    "db6033b7-fede-41ea-8c33-63bd31a68be7",
];

#[riven_test]
async fn val_match_v1_get_test() -> Result<(), String> {
    val_match_v1_get(ROUTE, MATCHES).await
}

#[riven_test]
async fn val_content_ranked_test() -> Result<(), String> {
    val_content_ranked(ROUTE).await
}

#[riven_test]
async fn val_match_v1_latest_test() -> Result<(), String> {
    val_match_v1_latest(ROUTE).await
}

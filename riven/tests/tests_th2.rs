mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::TH2;

#[riven_test]
async fn status() -> Result<(), String> {
    let p = riot_api().lol_status_v4().get_platform_data(ROUTE);
    let status = p.await.map_err(|e| e.to_string())?;
    let _ = status;
    Ok(())
}

#[riven_test]
async fn spectator_v5_combo_test() -> Result<(), String> {
    spectator_v5_combo(ROUTE).await
}

#[riven_test]
async fn spectator_tft_v5_combo_test() -> Result<(), String> {
    spectator_tft_v5_combo(ROUTE).await
}

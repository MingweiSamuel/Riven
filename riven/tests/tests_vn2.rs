mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::VN2;

#[riven_test]
async fn status() -> Result<(), String> {
    let p = riot_api().lol_status_v4().get_platform_data(ROUTE);
    let status = p.await.map_err(|e| e.to_string())?;
    let _ = status;
    Ok(())
}

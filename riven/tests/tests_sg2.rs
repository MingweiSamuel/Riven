mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::SG2;

#[tokio_shared_rt::test]
async fn status() -> Result<(), String> {
    let p = RIOT_API.lol_status_v4().get_platform_data(ROUTE);
    let status = p.await.map_err(|e| e.to_string())?;
    println!("{:?}", status);
    Ok(())
}

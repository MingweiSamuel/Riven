mod testutils;

use futures::future::join_all;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::TR1;

#[riven_test]
async fn league_summoner_bulk_test() -> Result<(), String> {
    let p = riot_api()
        .league_v4()
        .get_challenger_league(ROUTE, QueueType::RANKED_SOLO_5x5);
    let league_list = p.await.map_err(|e| e.to_string())?;

    println!(
        "{:?} Challenger {} entries.",
        ROUTE,
        league_list.entries.len()
    );

    let account_vec = join_all(league_list.entries.iter().take(50).map(|entry| async move {
        let account = riot_api()
            .account_v1()
            .get_by_puuid(ROUTE.to_regional(), &entry.puuid)
            .await;
        Ok(account)
    }))
    .await;

    for (i, s) in account_vec.into_iter().enumerate() {
        let account = s
            .and_then(std::convert::identity)
            .map_err(|e| e.to_string())?;
        println!(
            "{}: {}#{}",
            i + 1,
            account.game_name.unwrap_or_default(),
            account.tag_line.unwrap_or_default(),
        );
    }

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

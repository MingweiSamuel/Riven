mod testutils;
use futures::future::join_all;
use riven::consts::*;
use riven::models::summoner_v4::Summoner;
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

    let summoner_vec = join_all(league_list.entries.iter().take(50).map(|entry| {
        riot_api()
            .summoner_v4()
            .get_by_summoner_id(ROUTE, &entry.summoner_id)
    }))
    .await;

    for (i, s) in summoner_vec.into_iter().enumerate() {
        let summoner: Summoner = s.map_err(|e| e.to_string())?;
        println!("{}: {}", i + 1, summoner.name);
    }

    Ok(())
}

#[riven_test]
async fn spectator_v4_combo_test() -> Result<(), String> {
    spectator_v4_combo(ROUTE).await
}

#[riven_test]
async fn spectator_v5_combo_test() -> Result<(), String> {
    spectator_v5_combo(ROUTE).await
}

#[riven_test]
async fn spectator_tft_v5_combo_test() -> Result<(), String> {
    spectator_tft_v5_combo(ROUTE).await
}

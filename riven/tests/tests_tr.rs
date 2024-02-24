mod testutils;
use riven::consts::*;
use riven::models::summoner_v4::Summoner;
use testutils::RIOT_API;

const ROUTE: PlatformRoute = PlatformRoute::TR1;

#[tokio_shared_rt::test]
async fn league_summoner_bulk_test() -> Result<(), String> {
    let p = RIOT_API
        .league_v4()
        .get_challenger_league(ROUTE, QueueType::RANKED_SOLO_5x5);
    // let p = future_start(p);
    let ll = p.await.map_err(|e| e.to_string())?;

    println!("{:?} Challenger {} entries.", ROUTE, ll.entries.len());

    let sl = ll
        .entries
        .iter()
        .take(50)
        .map(|entry| {
            RIOT_API
                .summoner_v4()
                .get_by_summoner_id(ROUTE, &entry.summoner_id)
        })
        .map(tokio::spawn)
        .collect::<Vec<_>>();

    for (i, s) in sl.into_iter().enumerate() {
        let summoner: Summoner = s
            .await
            .expect("tokio::spawn join error")
            .map_err(|e| e.to_string())?;
        println!("{}: {}", i + 1, summoner.name);
    }

    Ok(())
}

#![feature(custom_test_frameworks)]
#![test_runner(my_runner)]

mod async_tests;
mod testutils;
use testutils::{ RIOT_API, future_start };

use colored::*;

use riven::consts::*;
use riven::endpoints::summoner_v4::Summoner;

const REGION: Region = Region::TR;


async_tests!{
    my_runner {
        league_summoner_bulk_test: async {
            let p = RIOT_API.league_v4().get_challenger_league(REGION, QueueType::RANKED_SOLO_5x5);
            // let p = future_start(p);
            let ll = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get challenger league".to_owned())?;

            println!("{} Challenger {} entries.", REGION.key, ll.entries.len());

            let sl = ll.entries[..50].iter()
                .map(|entry| RIOT_API.summoner_v4().get_by_summoner_id(REGION, &entry.summoner_id))
                .map(future_start)
                .collect::<Vec<_>>();

            for (i, s) in sl.into_iter().enumerate() {
                let summoner_opt: Option<Summoner> = s.await.map_err(|e| e.to_string())?;
                let summoner = summoner_opt.ok_or("Failed to get summoner.".to_owned())?;
                println!("{}: {}", i + 1, summoner.name);
            }

            Ok(())
        },
    }
}

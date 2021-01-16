#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::RIOT_API;

use colored::*;

use riven::consts::*;

const REGION: Region = Region::VAL_LATAM;


async_tests!{
    my_runner {
        val_content_ranked_test: async {
            let p = RIOT_API.val_content_v1().get_content(REGION, Some("zh-CN"));
            let contents = p.await.map_err(|e| e.to_string())?;

            let act = contents.acts.iter().find(|act| act.is_active)
                .ok_or(format!("No active acts of {} found.", contents.acts.len()))?;

            let p = RIOT_API.val_ranked_v1().get_leaderboard(REGION, &act.id, None, None);
            let leaderboard = p.await.map_err(|e| e.to_string())?
                .ok_or(format!("Failed to get act leaderboard {} {}.", act.id, act.name))?;

            rassert_eq!(act.id, leaderboard.act_id);

            for (i, p) in leaderboard.players.iter().take(10).enumerate() {
                rassert_eq!(i + 1, p.leaderboard_rank as usize);
                println!("{:>2}: {:>4}   {:<22} ({} wins)", p.leaderboard_rank, p.ranked_rating, format!("{}#{}", p.game_name, p.tag_line), p.number_of_wins);
            }

            Ok(())
        },
    }
}

#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::RIOT_API;

use colored::*;

use riven::consts::*;

const ROUTE: PlatformRoute = PlatformRoute::LA1;

async_tests! {
    my_runner {
        lol_challenges_v1_check_configs: async {
            let challenges = RIOT_API.lol_challenges_v1().get_all_challenge_configs(ROUTE)
                .await.map_err(|e| e.to_string())?;
            rassert!(!challenges.is_empty());

            for challenge in challenges.iter() {
                rassert!(!challenge.localized_names.is_empty());
                rassert!(!challenge.thresholds.is_empty());
            }

            // Spot-check 10% of the challenge IDs.
            for challenge in challenges.iter().step_by(10) {
                RIOT_API.lol_challenges_v1().get_challenge_configs(ROUTE, challenge.id)
                    .await.map_err(|e| format!("Failed to get challenge config with id {}\n{}", challenge.id, e))?
                    .ok_or_else(|| format!("Challenge config with id {} returned 404", challenge.id))?;
            }

            Ok(())
        },
        lol_challenges_v1_check_percentiles: async {
            // Check all percentiles.
            let percentiles = RIOT_API.lol_challenges_v1().get_all_challenge_percentiles(ROUTE)
                .await.map_err(|e| e.to_string())?;
            rassert!(!percentiles.is_empty());

            // Spot-check 10% of the challenge IDs.
            for &challenge_id in percentiles.keys().step_by(10) {
                RIOT_API.lol_challenges_v1().get_challenge_percentiles(ROUTE, challenge_id)
                    .await.map_err(|e| format!("Failed to get challenge percentile with id {}\n{}", challenge_id, e))?
                    .ok_or_else(|| format!("Challenge percentile with id {} returned 404", challenge_id))?;
            }

            Ok(())
        },
    }
}

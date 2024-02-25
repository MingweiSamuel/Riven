mod testutils;
use riven::consts::*;
use testutils::{riot_api, riven_test};

const ROUTE: PlatformRoute = PlatformRoute::LA1;

/// en_US description: "As a laner, get kills before 10 minutes outside your lane (anyone but your lane opponent)"
const CHALLENGE_ID_ARAM_1K_DPM: i64 = 101101;

/// /lol/challenges/v1/challenges/{challengeId}/leaderboards/by-level/{level}
/// /lol/challenges/v1/player-data/{puuid}
#[riven_test]
async fn lol_challenges_v1_leaderboards_playerdata() -> Result<(), String> {
    let challenge_id = CHALLENGE_ID_ARAM_1K_DPM;
    let leaderboard = riot_api()
        .lol_challenges_v1()
        .get_challenge_leaderboards(ROUTE, challenge_id, Tier::GRANDMASTER, None)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| {
            format!(
                "Challenge leaderboard with id {} returned 404",
                challenge_id
            )
        })?;

    {
        rassert!(!leaderboard.is_empty());
        let start = leaderboard[0].position;
        // Commented out: leaderboard is not monotonic for some reason.
        // let mut val = leaderboard[0].value;
        for (n, entry) in leaderboard.iter().enumerate() {
            rassert_eq!(start + (n as i32), entry.position);
            // rassert!(entry.val <= val);
            // val = etnry.val;
        }
    }

    // Spot check 10% for `player-data`.
    for entry in leaderboard.iter().step_by(10) {
        let _player_data = riot_api()
            .lol_challenges_v1()
            .get_player_data(ROUTE, &entry.puuid)
            .await
            .map_err(|e| format!("Failed to get player data PUUID {}: {}", entry.puuid, e))?;
    }

    Ok(())
}

/// /lol/challenges/v1/challenges/config
/// /lol/challenges/v1/challenges/{challengeId}/config
#[riven_test]
async fn lol_challenges_v1_check_configs() -> Result<(), String> {
    let challenges = riot_api()
        .lol_challenges_v1()
        .get_all_challenge_configs(ROUTE)
        .await
        .map_err(|e| e.to_string())?;
    rassert!(!challenges.is_empty());

    for challenge in challenges.iter() {
        rassert!(!challenge.localized_names.is_empty());
        rassert!(!challenge.thresholds.is_empty());
    }

    // Spot-check 10% of the challenge IDs.
    for challenge in challenges.iter().step_by(10) {
        riot_api()
            .lol_challenges_v1()
            .get_challenge_configs(ROUTE, challenge.id)
            .await
            .map_err(|e| {
                format!(
                    "Failed to get challenge config with id {}\n{}",
                    challenge.id, e
                )
            })?
            .ok_or_else(|| format!("Challenge config with id {} returned 404", challenge.id))?;
    }

    Ok(())
}

/// /lol/challenges/v1/challenges/percentiles
/// /lol/challenges/v1/challenges/{challengeId}/percentiles
#[riven_test]
async fn lol_challenges_v1_check_percentiles() -> Result<(), String> {
    // Check all percentiles.
    let percentiles = riot_api()
        .lol_challenges_v1()
        .get_all_challenge_percentiles(ROUTE)
        .await
        .map_err(|e| e.to_string())?;
    rassert!(!percentiles.is_empty());

    // Spot-check 10% of the challenge IDs.
    for &challenge_id in percentiles.keys().step_by(10) {
        riot_api()
            .lol_challenges_v1()
            .get_challenge_percentiles(ROUTE, challenge_id)
            .await
            .map_err(|e| {
                format!(
                    "Failed to get challenge percentile with id {}\n{}",
                    challenge_id, e
                )
            })?
            .ok_or_else(|| format!("Challenge percentile with id {} returned 404", challenge_id))?;
    }

    Ok(())
}

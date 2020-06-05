#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;


async_tests!{
    my_runner {
        // Summoner tests.
        summoner_get_kanjikana: async {
            let p = RIOT_API.summoner_v4().get_by_summoner_name(Region::JP, "私の 頭が かたい");
            let s = p.await.map_err(|e| e.to_string())?.ok_or("Failed to get myheadhard".to_owned())?;
            rassert_eq!("私の頭がかたい", s.name);
            Ok(())
        },

        // Failure cases.
        // Make sure get_raw_response(...) with invalid path fails as expected.
        raw_response_invalid: async {
            let p = RIOT_API.get_raw_response("summoner-v4.getBySummonerName", Region::JP.into(), "INVALID/PATH".to_owned(), None);
            let r = p.await;
            rassert!(r.is_err());
            Ok(())
        },
        // summoner_v4().get_by_summoner_name(...) normally returns an option.
        // If we use `get` (instead of `get_optional`) make sure it errors.
        get_nonoptional_invalid: async {
            let path_string = format!("/lol/summoner/v4/summoners/by-name/{}", "SUMMONER THAT DOES NOT EXIST");
            let p = RIOT_API.get::<riven::models::summoner_v4::Summoner>(
                "summoner-v4.getBySummonerName", Region::JP.into(), path_string, None);
            let r = p.await;
            rassert!(r.is_err());
            Ok(())
        },
        // Make sure 403 is handled as expected.
        tournament_forbidden: async {
            let p = RIOT_API.tournament_v4().get_tournament_code(Region::JP, "INVALID_CODE");
            let r = p.await;
            rassert!(r.is_err());
            rassert_eq!(Some(reqwest::StatusCode::FORBIDDEN), r.unwrap_err().status_code());
            Ok(())
        },
    }
}

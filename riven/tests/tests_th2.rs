#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use colored::*;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::TH2;

async_tests! {
    my_runner {
        status: async {
            let p = RIOT_API.lol_status_v4().get_platform_data(ROUTE);
            let status = p.await.map_err(|e| e.to_string())?;
            println!("{:?}", status);
            Ok(())
        },
    }
}

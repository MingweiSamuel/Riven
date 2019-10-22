#![feature(custom_test_frameworks)]
#![test_runner(my_runner)]

mod async_tests;

use colored::*;
use lazy_static::lazy_static;
use riven::RiotApi;
use tokio::runtime::current_thread::Runtime;

lazy_static! {
    static ref RIOT_API: RiotApi = {
        let api_key = std::fs::read_to_string("apikey.txt").unwrap(); // TODO don't use unwrap.
        RiotApi::with_key(api_key.trim())
    };
}

async_tests!{
    my_runner {
        test_1: async {
            rassert_eq!("world", "world");
            Ok(())
        },
        test_2: async {
            rassert_eq!("hello", "hello");
            Ok(())
        },
    }
}

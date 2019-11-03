#![allow(dead_code)]

use std::future::Future;

use futures_util::future::RemoteHandle;
use lazy_static::lazy_static;
use tokio::executor::{ DefaultExecutor, Executor };

use riven::{ RiotApi, RiotApiConfig };

lazy_static! {
    pub static ref RIOT_API: RiotApi = {
        let api_key = std::env::var("RGAPI_KEY").ok()
            .or_else(|| std::fs::read_to_string("apikey.txt").ok())
            .expect("Failed to find RGAPI_KEY env var or apikey.txt.");
        RiotApi::with_config(RiotApiConfig::with_key(api_key.trim())
            .preconfig_burst())
    };
}

pub fn future_start<Fut>(future: Fut) -> RemoteHandle<<Fut as Future>::Output>
where
    Fut: Future + Send + 'static,
    <Fut as Future>::Output: Send,
{
    Executor::spawn_with_handle(&mut DefaultExecutor::current(), future)
        .expect("Failed to spawn.")
}

pub mod ids {
    pub const SUMMONER_ID_LUGNUTSK: &'static str = "SBM8Ubipo4ge2yj7bhEzL7yvV0C9Oc1XA2l6v5okGMA_nCw";
    pub const SUMMONER_ID_MA5TERY:  &'static str = "IbC4uyFEEW3ZkZw6FZF4bViw3P1EynclAcI6-p-vCpI99Ec";
    pub const SUMMONER_ID_C9SNEAKY: &'static str = "ghHSdADqgxKwcRl_vWndx6wKiyZx0xKQv-LOhOcU5LU";
    pub const ACCOUNT_ID_C9SNEAKY:  &'static str = "ML_CcLT94UUHp1iDvXOXCidfmzzPrk_Jbub1f_INhw";
    pub const ACCOUNT_ID_LUGNUTSK:  &'static str = "iheZF2uJ50S84Hfq6Ob8GNlJAUmBmac-EsEEWBJjD01q1jQ";
}

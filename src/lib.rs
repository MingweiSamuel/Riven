pub mod consts;

mod riot_api_config;
pub use riot_api_config::*;

mod riot_api;
pub use riot_api::*;

mod req;
mod util;

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;
    use super::*;

    const API_KEY: &'static str = "RGAPI-nothinghereowo";

    #[test]
    fn it_works() {
        let rt = Runtime::new().unwrap();
        let riot_api = RiotApi::with_key(API_KEY);

        // https://na1.api.riotgames.com/lol/champion-mastery/v4/scores/by-summoner/SBM8Ubipo4ge2yj7bhEzL7yvV0C9Oc1XA2l6v5okGMA_nCw
        let my_future = riot_api.get::<u32>("asdf", consts::Region::NA,
            "/lol/champion-mastery/v4/scores/by-summoner/SBM8Ubipo4ge2yj7bhEzL7yvV0C9Oc1XA2l6v5okGMA_nCw",
            &[]);
        let val = rt.block_on(my_future).unwrap();
        println!("VAL: {}", val.unwrap());
    }
}

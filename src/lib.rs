include!("../srcgen/mod.rs");

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

    #[test]
    fn it_works() {
        env_logger::init();

        let champ = crate::consts::Champion::Riven;
        println!("{}", champ);

        let api_key_raw = std::fs::read_to_string("apikey.txt").unwrap(); // TODO don't use unwrap.
        let api_key = api_key_raw.trim();

        let rt = Runtime::new().unwrap();
        let riot_api = RiotApi::with_key(api_key);

        for i in 0..2 {
            // https://na1.api.riotgames.com/lol/champion-mastery/v4/scores/by-summoner/SBM8Ubipo4ge2yj7bhEzL7yvV0C9Oc1XA2l6v5okGMA_nCw
            let my_future = riot_api.get::<u32>("asdf", consts::Region::NA,
                "/lol/champion-mastery/v4/scores/by-summoner/SBM8Ubipo4ge2yj7bhEzL7yvV0C9Oc1XA2l6v5okGMA_nCw",
                &[]);
            let val = rt.block_on(my_future).unwrap();
            println!("VAL {}: {}", i, val.unwrap());
        }
    }
}

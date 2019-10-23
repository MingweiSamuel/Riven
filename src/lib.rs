//! Module docs TODO.

mod riot_api_error;
pub use riot_api_error::*;

pub mod consts;

pub mod endpoints;

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

    use url::form_urlencoded::Serializer;
    #[test]
    fn checkme() {
        let mut query = Serializer::new(String::new());
        query.append_pair("hello", "false");
        query.append_pair("hello", "world");
        let result = query.finish();
        println!("{}", result);
    }

    #[test]
    #[ignore]
    fn it_works() {
        env_logger::init();

        let champ = crate::consts::Champion::Riven;
        println!("{}", champ);

        let api_key = std::fs::read_to_string("apikey.txt").unwrap(); // TODO don't use unwrap.

        let rt = Runtime::new().unwrap();
        let riot_api = RiotApi::with_key(api_key.trim());

        for i in 0..2 {
            let my_future = riot_api.champion_mastery_v4().get_all_champion_masteries(
                consts::Region::NA, "SBM8Ubipo4ge2yj7bhEzL7yvV0C9Oc1XA2l6v5okGMA_nCw");
            let val = rt.block_on(my_future).unwrap();
            //println!("VAL {}: {:#?}", i, val.unwrap());
        }
    }
}

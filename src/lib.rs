#![allow(dead_code)] // TODO REMOVE

pub mod consts;

mod req;
mod riot_api_config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

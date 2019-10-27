#![cfg(test)]

use std::time::Duration;

use fake_clock::FakeClock as Instant;

mod token_bucket {
    include!("token_bucket.rs");

    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let _x = VectorTokenBucket::new(Duration::from_secs(1), 100);
        }
    }
}

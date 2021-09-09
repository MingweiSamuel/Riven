//! Module containing rate limiting and requesting types.

mod rate_limit;
pub use rate_limit::*;

mod rate_limit_type;
pub use rate_limit_type::*;

use std::time::Instant; // Hack for token_bucket_test.rs.
mod token_bucket;
pub use token_bucket::*;

mod regional_requester;
pub use regional_requester::*;

#[cfg(test)]
#[path = "token_bucket.test.rs"]
mod token_bucket_test;

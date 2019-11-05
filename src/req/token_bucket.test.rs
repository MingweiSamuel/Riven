#![cfg(test)]

use fake_instant::FakeClock as Instant;

/// This is a hack to test token bucket, substituting FakeClock for Instant.
mod token_bucket {
    include!("token_bucket.rs");

    mod tests {
        use super::*;
        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref D00: Duration = Duration::new(0, 0);
        }

        #[test]
        fn test_basic() {
            Instant::set_time(50_000);
            let bucket = VectorTokenBucket::new(Duration::from_millis(1000), 100, *D00, 0.95);
            assert!(bucket.get_tokens(50), "Should have not violated limit.");
            assert_eq!(None, bucket.get_delay(), "Can get stuff.");
            assert!(!bucket.get_tokens(51), "Should have violated limit.");
        }

        #[test]
        fn test_internal_constructor() {
            let bucket = VectorTokenBucket::new(Duration::from_millis(1000), 100, *D00, 1.0);
            assert_eq!(100, bucket.burst_limit);

            let bucket = VectorTokenBucket::new(Duration::from_millis(1000), 100, *D00, 1e-6);
            assert_eq!(1, bucket.burst_limit);
        }

        #[test]
        fn test_saturated_100_burst() {
            let bucket = VectorTokenBucket::new(Duration::from_millis(1000), 100, *D00, 1.00);

            Instant::set_time(50_000);
            assert!(bucket.get_tokens(100), "All tokens should be immediately available.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(1001); // Extra buffer for Duration(0).
            assert!(bucket.get_tokens(100), "All tokens should be available after a bucket duration.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");
        }

        #[test]
        fn test_saturated_95_burst() {
            let bucket = VectorTokenBucket::new(Duration::from_millis(1000), 100, *D00, 0.50);

            Instant::set_time(50_000);
            assert!(bucket.get_tokens(95), "95 tokens should be immediately available.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(475); // Total 951.
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(476); // Extra buffer for Duration(0).
            assert!(bucket.get_tokens(5), "Last 5 tokens should be available.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(51);
            assert!(bucket.get_tokens(95), "95 tokens should be available.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(951);
            assert!(bucket.get_tokens(5), "Last 5 tokens should be available.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");
        }

        #[test]
        fn test_saturated_50_burst() {
            let bucket = VectorTokenBucket::new(Duration::from_millis(1000), 100, *D00, 0.5);

            Instant::set_time(50_000);
            assert!(bucket.get_tokens(50), "Half the tokens should be immediately available.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(501); // Extra buffer for Duration(0).
            assert!(bucket.get_tokens(50), "Half the tokens should be available after a half bucket duration.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(501);
            assert!(bucket.get_tokens(50), "Half the tokens should be available after a full bucket duration.");
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");

            Instant::advance_time(501);
            assert!(bucket.get_tokens(50));
            assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");
        }

        #[test]
        fn test_many() {
            Instant::set_time(50_000);
            let bucket = VectorTokenBucket::new(Duration::from_millis(1000), 100, *D00, 0.95);
            assert!(bucket.get_tokens(50), "Should have not violated limit.");
            assert_eq!(None, bucket.get_delay(), "Should not be blocked.");
            for _ in 0..20_000 {
                Instant::advance_time(501);
                assert!(bucket.get_tokens(50), "Should have not violated limit.");
                assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");
                Instant::advance_time(501);
                assert!(bucket.get_tokens(50), "Should have not violated limit.");
                assert_ne!(None, bucket.get_delay(), "Bucket should have delay.");
            }
            assert!(bucket.timestamps.lock().len() < 110, "Check memory leak.");
        }
    }
}

use std::future::Future;

use crate::time::Instant;

/// Returns a wrapped future that records the time it takes to complete the future as a histogram metric.
pub fn timed<Fut>(
    future: Fut,
    operation: &'static str,
    route: &'static str,
) -> impl Future<Output = Fut::Output>
where
    Fut: Future,
{
    async move {
        let start = Instant::now();
        let out = future.await;
        metrics::histogram!("riot_api", "operation" => operation, "route" => route)
            .record(start.elapsed());
        out
    }
}

use std::future::Future;

use crate::time::Instant;

/// Returns a wrapped future that records the time it takes to complete the future as a histogram metric.
pub async fn timed<Fut>(future: Fut, operation: &'static str, route: &'static str) -> Fut::Output
where
    Fut: Future,
{
    let start = Instant::now();
    let out = future.await;
    metrics::histogram!("riot_api", "operation" => operation, "route" => route)
        .record(start.elapsed());
    out
}

/// Returns a wrapped future that records the time it takes to complete the future as a histogram metric.
pub async fn try_timed<Fut, T, E>(
    future: Fut,
    operation: &'static str,
    route: &'static str,
) -> Result<T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    let start = Instant::now();
    let result = future.await;
    if result.is_ok() {
        metrics::histogram!("riot_api", "operation" => operation, "route" => route)
            .record(start.elapsed());
    }
    result
}

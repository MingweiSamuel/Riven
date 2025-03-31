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
pub async fn try_timed<Fut, T>(
    future: Fut,
    operation: &'static str,
    route: &'static str,
) -> Option<T>
where
    Fut: Future<Output = Option<T>>,
{
    let start = Instant::now();
    let out = future.await;
    if out.is_some() {
        metrics::histogram!("riot_api", "operation" => operation, "route" => route)
            .record(start.elapsed());
    }
    out
}

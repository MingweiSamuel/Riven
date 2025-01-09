use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use pin_project::pin_project;
use tracing::info;

#[pin_project]
/// Wraps an inner Future, captures operation + route and records the time it takes to complete
/// as a histogram metric.
struct TimedMetric<Fut>
where
    Fut: Future,
{
    #[pin]
    inner: Fut,
    operation: &'static str,
    route: &'static str,
    start: Option<Instant>,
}

impl<Fut> Future for TimedMetric<Fut>
where
    Fut: Future,
{
    type Output = Fut::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let start = this.start.get_or_insert_with(Instant::now);

        match this.inner.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(v) => {
                metrics::histogram!("riot_api", "operation" => *this.operation, "route" => *this.route).record(start.elapsed());
                Poll::Ready(v)
            }
        }
    }
}

/// Returns a wrapped future that records the time it takes to complete the future as a histogram metric.
pub fn timed<Fut>(
    future: Fut,
    operation: &'static str,
    route: &'static str,
) -> impl Future<Output = Fut::Output>
where
    Fut: Future,
{
    TimedMetric {
        inner: future,
        operation,
        route,
        start: None,
    }
}

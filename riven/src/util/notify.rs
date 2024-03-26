use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use futures::future::FusedFuture;
use futures::FutureExt;
use parking_lot::Mutex;
use slab::Slab;

#[derive(Default)]
pub struct Notify {
    internal: Mutex<NotifyInternal>,
}

#[derive(Default)]
struct NotifyInternal {
    /// Incremented each time [`Self::notify_waiters()`] is called.
    pub generation: usize,
    /// List of waiters.
    pub waiters: Slab<Waker>,
}

impl Notify {
    /// Creates a new `Notify` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a future which waits for a notification via [`Self::notify_wakers`].
    ///
    /// Dropping the returned future will de-register it from this `Notify` instance, which
    /// [prevents memory leaks](https://github.com/MingweiSamuel/Riven/pull/67).
    pub fn notified(&self) -> impl '_ + FusedFuture<Output = ()> {
        struct Notified<'a> {
            /// Parent notify reference.
            notify: &'a Notify,
            /// Generation of this notify. To prevent the ABA problem with `slab` keys.
            /// Starts out `None`, set to the generation once the `Waker` is registered into [`NotifyInternal::waiters`].
            generation_and_key: Option<(usize, usize)>,
        }
        impl Future for Notified<'_> {
            type Output = ();

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                if let Some(_generation_and_key) = self.generation_and_key.take() {
                    // Already registered, this is waking via `notify_wakers` (probably).
                    // `generation_and_key.take()` to set back to `None`, avoid extra `Drop` work.
                    // Ok since we call `fuse()` to prevent re-polling after this return.
                    Poll::Ready(())
                } else {
                    // Register and set the generation (to preven ABA problem).
                    let mut internal = self.notify.internal.lock();
                    let key = internal.waiters.insert(cx.waker().clone());
                    self.generation_and_key = Some((internal.generation, key));
                    Poll::Pending
                }
            }
        }
        impl Drop for Notified<'_> {
            fn drop(&mut self) {
                // Only bother deallocating if registered (i.e. `generation_and_key` is set).
                if let Some((generation, key)) = self.generation_and_key {
                    let mut internal = self.notify.internal.lock();
                    // Ensure generation matches before removing, to prevent ABA problem.
                    // If no match, means `Notify::notify_waiters` has already been called and deallocated us.
                    if internal.generation == generation {
                        internal
                            .waiters
                            .try_remove(key)
                            .expect("Expected to drop registered `Notified` but waker not found.");
                        internal.waiters.shrink_to_fit();
                    }
                }
            }
        }

        Notified {
            notify: self,
            generation_and_key: None,
        }
        .fuse()
    }

    /// Notifies all waiting tasks.
    pub fn notify_waiters(&self) {
        let mut internal = self.internal.lock();
        // Increment generation when we clear the slab.
        // Wrap, although not likely to matter. ABBB...BA problem with `usize::MAX` 'B's.
        internal.generation = internal.generation.wrapping_add(1);
        internal.waiters.drain().for_each(Waker::wake);
        internal.waiters.shrink_to_fit();
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod test {
    use futures::FutureExt;

    use super::*;

    #[tokio::test]
    async fn memory_leak() {
        let notify = Notify::new();

        for _ in 0..100 {
            futures::select_biased! {
                _ = notify.notified() => {}
                _ = std::future::ready(()).fuse() => {}
            }
        }

        let internal = notify.internal.lock();
        assert_eq!(0, internal.waiters.len());
        assert_eq!(0, internal.waiters.capacity());
    }
}

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use parking_lot::Mutex;

#[derive(Default)]
pub struct Notify {
    waiters: Mutex<Vec<Waker>>,
}
impl Notify {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn notified(&self) -> impl '_ + Future<Output = ()> {
        struct Notified<'a> {
            notify: &'a Notify,
            registered: bool,
        }
        impl Future for Notified<'_> {
            type Output = ();

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                if std::mem::replace(&mut self.as_mut().registered, true) {
                    Poll::Ready(())
                } else {
                    self.notify.waiters.lock().push(cx.waker().clone());
                    Poll::Pending
                }
            }
        }

        Notified {
            notify: self,
            registered: false,
        }
    }

    pub fn notify_waiters(&self) {
        self.waiters.lock().drain(..).for_each(Waker::wake);
    }
}

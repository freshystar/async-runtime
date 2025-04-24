use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    task::{Context, Poll, Waker},
};

pub struct Task {
    pub future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    pub is_done: Arc<AtomicBool>,
}

impl Task {
    pub fn new(fut: impl Future<Output = ()> + Send + 'static) -> Arc<Self> {
        Arc::new(Self {
            future: Mutex::new(Some(Box::pin(fut))),
            is_done: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn poll(self: Arc<Self>, waker: &Waker) -> bool {
        let mut future = self.future.lock().unwrap();
        if let Some(mut fut) = future.take() {
            let mut cx = Context::from_waker(waker);
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(_) => {
                    self.is_done.store(true, Ordering::SeqCst);
                    true
                }
                Poll::Pending => {
                    *future = Some(fut);
                    false
                }
            }
        } else {
            true
        }
    }
}

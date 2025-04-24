use crate::handler::task::Task;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    task::{Context, Poll},
};

pub struct JoinHandle {
    done: Arc<AtomicBool>,
}

impl JoinHandle {
    pub fn new(task: &Arc<Task>) -> Self {
        JoinHandle {
            done: task.is_done.clone(),
        }
    }
}

impl Future for JoinHandle {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.done.load(Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

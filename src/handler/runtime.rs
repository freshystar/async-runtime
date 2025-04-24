use crate::{handler::task::Task, handler::join_handle::JoinHandle, handler::waker::dummy_waker};
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::Duration,
};

#[derive(Clone)]
pub struct MiniRuntime {
    queue: Arc<Mutex<VecDeque<Arc<Task>>>>,
}


impl MiniRuntime {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Task::new(future);
        let handle = JoinHandle::new(&task);
        self.queue.lock().unwrap().push_back(task);
        handle
    }

    pub fn block_on<F: Future>(&self, mut future: F) -> F::Output {
        let waker = dummy_waker();
        let mut cx = Context::from_waker(&waker);
        let mut future = unsafe { Pin::new_unchecked(&mut future) };

        loop {
            if let Poll::Ready(val) = future.as_mut().poll(&mut cx) {
                return val;
            }

            let tasks = self.queue.lock().unwrap().drain(..).collect::<Vec<_>>();
            for task in tasks {
                let keep = task.clone();
                if !task.poll(&waker) {
                    self.queue.lock().unwrap().push_back(keep);
                }
            }

            thread::sleep(Duration::from_millis(1));
        }
    }
}

// Global Runtime Access for spawn() 
use std::cell::RefCell;

thread_local! {
    static GLOBAL_RUNTIME: RefCell<Option<Arc<MiniRuntime>>> = RefCell::new(None);
}

pub fn set_global(rt: Arc<MiniRuntime>) {
    GLOBAL_RUNTIME.with(|cell| {
        *cell.borrow_mut() = Some(rt);
    });
}


pub fn with_global<F: FnOnce(&MiniRuntime) -> R, R>(f: F) -> R {
    GLOBAL_RUNTIME.with(|cell| {
        f(cell.borrow().as_ref().expect("Global runtime not set"))
    })
}


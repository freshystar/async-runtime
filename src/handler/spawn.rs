use crate::{handler::join_handle::JoinHandle, handler::runtime::with_global};
use std::future::Future;

pub fn spawn<F>(fut: F) -> JoinHandle
where
    F: Future<Output = ()> + Send + 'static,
{
    with_global(|rt| rt.spawn(fut))
}

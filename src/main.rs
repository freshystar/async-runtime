mod handler;

use crate::handler::spawn::spawn;
use crate::handler::runtime::{set_global, MiniRuntime};
use crate::handler::sleep::sleep;
use std::{sync::Arc, time::Duration};

async fn task_one() {
    println!("task one: start");
    sleep(Duration::from_secs(1)).await;
    println!("task one: done✅");
}

async fn task_two() {
    println!("task two: start");
    sleep(Duration::from_secs(2)).await;
    println!("task two: done✅");
}

fn main() {
    let rt = Arc::new(MiniRuntime::new());
    set_global(rt.clone());

    rt.block_on(async {
        let _ = spawn(async {
            println!("Runtime started...");
        }).await;

        task_one().await;
        task_two().await;
    });
}

#  🔄 Minimal Async Runtime

MiniRuntime is a simple, single-threaded asynchronous runtime built in Rust. It supports spawning tasks, running them to completion, cooperative yielding, and sleeping for a given duration.

## 📚Features
- **Spawning Tasks**: Use spawn() to spawn async tasks.

- **Block Execution**: Use block_on() to run an async function until completion.

- **Sleeping**: sleep() simulates async sleeping for a given duration.

- **Join All**: The join_all!() macro to await multiple tasks simultaneously.

- **Cooperative Yielding**: yield_now() allows tasks to yield control to the runtime.
## 📋Installation
Clone ths repository and run the following command:
```sh
$ cd async-runtime
$ cargo run
```

#[macro_export]
macro_rules! join_all {
    ($($fut:expr),+ $(,)?) => {
        {
            $(let _ = $fut.await;)+
        }
    };
}


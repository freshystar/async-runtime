use std::{future::Future, pin::Pin, task::{Context, Poll}, time::Instant};

pub async fn sleep(duration: std::time::Duration) {
    let target = Instant::now() + duration;

    struct Sleep {
        target: Instant,
    }

    impl Future for Sleep {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if Instant::now() >= self.target {
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    Sleep { target }.await;
}

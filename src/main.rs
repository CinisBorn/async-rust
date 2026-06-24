use std::{io::Write};
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    let mut handlers = Vec::new();

    for t in 0..1_000 {
        // A spanw_blocking creates a new OS thread if there are no idle blocking thread available. 
        // 
        // A blocking operation is invoked in a separated pool. 
        // 
        // The limit of blocking thread is 512 blocking threads. If it surpass the value, a blocking
        // thread is put in a queue.
        // 
        // A blocking thread is similar to OS threads, but managed by tokio runtime. 
        let handle = spawn_blocking(async move || {
            let mut options = std::fs::OpenOptions::new()
                .create(true) 
                .write(true)
                .open(format!("temp/f{}", t))
                .unwrap();

            options.write(b"Good morning!").unwrap();
           
        });

        handlers.push(handle);
    };

    for h in handlers {
        h.await.unwrap().await;
    }

    println!("Finished");
}

struct Next {}

impl Future for Next {
    type Output = String;
    
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {

        let w = cx.waker();
        
        std::task::Poll::Pending
    }
}
#[tokio::main]
async fn main() {
    tokio::spawn(async move { 
        async_hello().await;
    });

    tokio::spawn(async move {
       async_world().await; 
    });

    tokio::spawn(async move {
        println!("Starting...");
    });
}

async fn async_world() {
    println!("world");
}
async fn async_hello() {
    println!("Hello");
}
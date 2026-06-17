#[tokio::main]
async fn main() {
    // without the await, nothing occurs. 
    async_world().await;
    async_hello().await;
   
    tokio::spawn(async move { println!("Read Vagabound manga!") }).await.unwrap();
}

async fn async_world() {
    println!("world");
}
async fn async_hello() {
    println!("Hello");
}
use mini_redis::{client,Result};

#[tokio::main]
// async fn main() -> Result<()> {
async fn main() -> Result<()>{
    // println!("Hello, world!");
    let mut client = client::connect("127.0.0.1:6379").await?;
    // set a key
    client.set("hello", "world".into()).await?;
    // get a key
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);
    Ok(())

}

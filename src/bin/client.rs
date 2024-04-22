use bytes::Bytes;
use mini_redis;
use tokio::sync::mpsc;
// define one customer and N procedure queue

#[derive(Debug)]
enum Command{
    Get {
        key:String
    },
    Set{
        key: String,
        val: Bytes,
    }
}

#[tokio::main]
async fn main(){
    let (tx,mut rx) = mpsc::channel(32);
    // define a channel with a buffer size of 32
    let tx2 =tx.clone();
    // clone the transmitter ,so that we can use it in the multiple tasks
    tokio::spawn(async move{
        tx.send("send from first task").await;
    });
    tokio::spawn(async move{
        tx2.send("send from second task").await;
    });

    // read from the channel
    while let Some(message) = rx.recv().await{
        println!("GOT = {:?}",message);
    }

}



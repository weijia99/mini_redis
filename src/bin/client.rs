use bytes::Bytes;
use mini_redis::{self, client};
use tokio::sync::{mpsc,oneshot};
// define one customer and N procedure queue
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command{
    Get {
        key:String,
        // resp:Responder<Option<Bytes>>
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
    let manager = tokio::spawn(async move{
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) =rx.recv().await  {
            use Command::*;
            match cmd {
                Get {key} => {
                    client.get(&key).await;
                }
                Set { key, val } =>{
                    client.set(&key,val);
                }
            }
            
        }
    });
    // add 2 tasks 
    let tx2 =tx.clone();
    let t1= tokio::spawn(async move{
        let cmd =Command::Get { key: "hello".to_string(), };
        tx.send(cmd).await.unwrap();
    });
    let t2 = tokio::spawn(async move{
        let cmd = Command::Set { key: "foo".to_string(), val: "bar".into(), };
        tx2.send(cmd).await.unwrap();
    });

    // In a certain order, wait for the tasks to complete
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

}



use bytes::Bytes;
use mini_redis::{self, client};
use tokio::sync::{mpsc,oneshot};
// define one customer and N procedure queue
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command{
    Get {
        key:String,
        resp:Responder<Option<Bytes>>
    },
    Set{
        key: String,
        val: Bytes,
        resp:Responder<()>
    }
}

#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel(32);
    // define a channel with a buffer size of 32
    let manager = tokio::spawn(async move{
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) =rx.recv().await  {
            use Command::*;
            match cmd {
                Get {key,resp} => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                    // send the response back to the requester
                    // use mpsc to send the response,then use oneshot to send the response back to the requester
                }
                Set { key, val,resp } =>{
                   let res = client.set(&key,val).await;
                   let _ = resp.send(res);
                }
            }
            
        }
    });
    // add 2 tasks 
    let tx2 =tx.clone();
    let t1= tokio::spawn(async move{
        let (resp_tx,resp_rx) = oneshot::channel();
        let cmd =Command::Get { key: "foo".to_string(),resp:resp_tx };
        if tx.send(cmd).await.is_err() {
            eprint!("connection task shutdown");
            return;
        }
        let res = resp_rx.await.unwrap();
        // wait for the response
        println!("GOT (Get) = {:?}", res);
    });
    let t2 = tokio::spawn(async move{
        let (resp_tx,resp_rx) = oneshot::channel();

        let cmd = Command::Set { key: "foo".to_string(), val: "bar".into(),resp:resp_tx };
        if tx2.send(cmd).await.is_err() {
            eprint!("connection task shutdown");
            return;
        }
        let res = resp_rx.await.unwrap();
        // wait for the response
        println!("GOT (Set) = {:?}", res);
    });

    // In a certain order, wait for the tasks to complete
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

}



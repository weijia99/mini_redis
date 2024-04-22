use std::os::windows::process;

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection,Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
type Db = Arc<Mutex<HashMap<String,Bytes>>>;
// Arc is used to share data between threads
#[tokio::main]
async fn main(){
    // bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    println!("Listening");
    let db = Arc::new(Mutex::new(HashMap::new()));
    
        loop{
            let (socket,_) =listener.accept().await.unwrap();
            // await will make situation to wait until the connection is made
            // process(socket).await;
            let db = db.clone();
            tokio::spawn(async move{
                process(socket,db).await;
                // this will spawn a new task to process the connection
            });
        }
}
async fn process(socket: TcpStream,db: Db){
    // let mut connection = Connection::new(socket);
    // if let Some(frame) =connection.read_frame().await.unwrap(){
    //     println!("GOT: {:?}",frame);
    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                // set lock to write to the db
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) =>{
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key())  {
                    Frame::Bulk(value.clone())
                    
                }else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}",cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
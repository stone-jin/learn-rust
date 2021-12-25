use mini_redis::Frame;
use mini_redis::Connection;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use mini_redis::{Result};
use std::collections::HashMap;
use mini_redis::Command::{self, Get, Set};

async fn process(tcpstream: TcpStream){
    // A hashmap is used to store data
    let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(tcpstream);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

async fn start_server(){
  let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
  loop{
    let (tcpstream, address) = listener.accept().await.unwrap();
    println!("address: {}", address);
    spawn(async move {
      process(tcpstream).await;
    });
  }
}

#[tokio::main]
async fn main() -> Result<()> {

    println!("in server");

    start_server().await;
    Ok(())
}

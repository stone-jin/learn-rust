use mini_redis::Frame;
use mini_redis::Connection;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use mini_redis::{Result};

async fn process(tcpstream: TcpStream){
  let mut connection = Connection::new(tcpstream);
  if let Some(frame) = connection.read_frame().await.unwrap(){
    println!("Got: {:?}", frame);
    let response = Frame::Error(String::from("not implments"));
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

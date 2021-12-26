
use tokio::io::AsyncReadExt;
use bytes::Buf;
use std::io::Cursor;
use mini_redis::Frame;
use bytes::BytesMut;
use tokio::net::TcpStream;
use mini_redis::{Result, client};
use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};
use tokio::io::AsyncWriteExt;

type Responder<T> = oneshot::Sender<Result<T>>;

struct Connection{
  stream: TcpStream,
  buffer: BytesMut
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection{
      Connection{
        stream,
        buffer: BytesMut::with_capacity(4096),
      }
    }
    pub async fn write_frame(&mut self, frame: &Frame) -> tokio::io::Result<()>{
      match frame{
        Frame::Simple(val) => {
          self.stream.write_u8(b'+').await?;
          self.stream.write_all(val.as_bytes()).await?;
          self.stream.write_all(b"\r\n").await?;
        }
        Frame::Error(val) => {
          self.stream.write_u8(b'-').await?;
          self.stream.write_all(val.as_bytes()).await?;
          self.stream.write_all(b"\r\n").await?;
        }
        Frame::Integer(val) =>{
          self.stream.write_u8(b':').await?;
          self.write_decimal(*val).await?;
        }
        Frame::Bulk(val) =>{
          let len = val.len();
          self.stream.write_u8(b'$').await?;
          self.write_decimal(len as u64).await?;
          self.stream.write_all(val).await?;
          self.stream.write_all(b"\r\n").await?;
        }
        Frame::Null => {
          self.stream.write_all(b"$-1\r\n").await?;
        }
        Frame::Array(_val) => {
          unimplemented!();
        }
      }

      self.stream.flush().await;

      Ok({})
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>>{
      loop{
        if let Some(frame) = self.parse_frame()? {
          return Ok(Some(frame));
        }

        if 0 == self.stream.read_buf(&mut self.buffer).await?{
          if self.buffer.is_empty(){
            return Ok(None);
          }else{
            return Err("connection peer by peer".into());
          }
        }
      }
    }

    pub async fn write_decimal(&mut self, val: u64)-> tokio::io::Result<()>{
      let mut buf = [0u8; 12];
      let mut buf = Cursor::new(&mut buf[..]);
      use std::io::Write;
      write!(&mut buf, "{}", val)?;


      let pos = buf.position() as usize;
      self.stream.write_all(&buf.get_ref()[..pos]).await?;
      self.stream.write_all(b"\r\n").await?;
      Ok({})
    }

    pub fn parse_frame(&mut self) -> Result<Option<Frame>> {
      let mut buf = Cursor::new(&self.buffer[..]);
      
      match Frame::check(&mut buf){
        Ok(_) =>{
          let len = buf.position() as usize;
          buf.set_position(0);

          let frame = Frame::parse(&mut buf)?;
          self.buffer.advance(len);
          Ok(Some(frame))
        }
        Err(Incompute) =>Ok(None),
        Err(e)=>Err(e.into())
      }
    }
}

#[derive(Debug)]
enum Command {
    Get {
      key: String,
      resp: Responder<Option<Bytes>>
    },
    Set{
      key: String,
      val: Bytes,
      resp: Responder<()>
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx
        };

        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("res: {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx ) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("set resp: {:?}", res);
    });

    let manager = tokio::spawn(async move {
      let mut client = client::connect("127.0.0.1:6379").await.unwrap();
      while let Some(cmd) = rx.recv().await {
        use Command::*;
        match cmd {
            Get { key, resp } => {
                let res = client.get(&key).await;
                let _ = resp.send(res).unwrap();
            }
            Set { key, val, resp } => {
                let res = client.set(&key, val).await;
                let _ = resp.send(res).unwrap();
            }
        }
      }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
    // println!("got value from the server; result={:?}", result);

    // jh.join();
    Ok(())
}

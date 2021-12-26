
use mini_redis::{Result, client};
use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<Result<T>>;

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

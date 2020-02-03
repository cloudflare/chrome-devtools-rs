use crate::Domain;

use futures::{
    future::{BoxFuture, FutureExt},
    stream::FuturesOrdered,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde_json;
use thiserror::Error;
use tokio::{net::TcpStream, time};
use tokio_tungstenite::{
    tungstenite::{self, Message},
    WebSocketStream,
};

use std::time::Duration;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("could not convert message to json")]
    Serialization(#[from] serde_json::Error),
    #[error("error sending message across websocket")]
    Tungstenite(#[from] tungstenite::Error),
    #[error("domain {0:?} is not yet implemented")]
    UnimplementedDomain(Domain),
}

pub struct Connection {
    write: SplitSink<WebSocketStream<TcpStream>, Message>,
    read: SplitStream<WebSocketStream<TcpStream>>,
    futures: FuturesOrdered<BoxFuture<'static, Result<(), ConnectionError>>>,
    id: u64,
}

impl Connection {
    pub fn new(stream: WebSocketStream<TcpStream>) -> Connection {
        let (write, read) = stream.split();
        Connection {
            write,
            read,
            futures: FuturesOrdered::new(),
            id: 0,
        }
    }

    pub async fn enable(&mut self, domain: Domain) -> Result<(), ConnectionError> {
        let message = match domain {
            Domain::Runtime(_) => Ok(crate::runtime::SendMethod::Enable(self.id.into())),
            _ => Err(ConnectionError::UnimplementedDomain(domain)),
        }?;
        self.id += 1;
        let json = serde_json::to_string(&message)?;
        let message = Message::Text(json);
        let _ = self.write.send(message).await?;
        Ok(())
    }

    pub fn send_heartbeat(&mut self, message: String, interval: u64) {
        let duration = Duration::from_millis(interval);
        let message = Message::Text(message);
        self.futures.push(
            async move {
                let mut interval = time::interval(duration);
                loop {
                    interval.tick().await;
                    self.write.send(message).await;
                }
            }
            .boxed(),
        );
    }
}

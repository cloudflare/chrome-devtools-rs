mod connection;
use connection::Connection;

use http::status::StatusCode;
use thiserror::Error;
use tokio_tungstenite;
use url::Url;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("could not parse websocket URL")]
    UrlParseError(#[from] url::ParseError),
    #[error("websocket endpoint {url} responded with status {status}")]
    SocketResponseError { url: Url, status: StatusCode },
    #[error("could not complete websocket handshake")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
}

#[derive(Clone)]
pub struct Client {
    url: Url,
}

impl Client {
    pub fn new(url: Url) -> Client {
        Client { url }
    }

    pub async fn connect(self) -> Result<Connection, ClientError> {
        let (stream, response) = tokio_tungstenite::connect_async(&self.url).await?;
        if !response.status().is_success() {
            return Err(ClientError::SocketResponseError {
                url: self.url,
                status: response.status(),
            });
        }

        Ok(Connection::new(stream))
    }
}

impl Default for Client {
    fn default() -> Self {
        let url = Url::parse("http://localhost:9222").expect("Could not parse default url");
        Self::new(url)
    }
}

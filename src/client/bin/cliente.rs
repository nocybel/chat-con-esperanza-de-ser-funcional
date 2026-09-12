use tokio::time::{sleep, Duration};
use futures_util::{SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct Cliente {
}

impl Cliente {
    pub fn new() -> Self {
        Cliente {
        }
    }

    pub async fn start(&self, ip: &str) {
        let (mut ws_stream, _) =
            connect_async(ip)
            .await
            .expect("no se pudo conectar");

        for i in 1..999999 {
            ws_stream.send(Message::Text(i.to_string().into())).await.unwrap();
            //sleep(Duration::from_millis(400)).await;
        }

        ws_stream.close(None).await.unwrap();
    }
}
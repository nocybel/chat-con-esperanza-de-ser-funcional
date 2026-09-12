use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

pub struct Usuario<'a> {
    _username: &'a str,
    //websocket: WebSocketStream<TcpStream>
}

impl Usuario<'_> {
    pub async fn new(listener: &TcpListener) -> Self {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("conexion: {addr}");

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (_write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                match message {
                    Ok(message) => {
                        if message.is_text() {
                            println!("[{}]: {}", addr, message.to_text().unwrap());
                        }
                    }
                    Err(e) => {
                        eprintln!("error de websocket: {e}");
                        break;
                    }
                }
            }

            println!("desconexion: {addr}");
        });
        
        Usuario {
            _username: "a"
        }
    }
}
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio::signal;

#[tokio::main]
async fn main() {
    println!("servidor");

    let ip = "127.0.0.1:8080";
    let listener = TcpListener::bind(ip)
        .await
        .expect("no se pudo dominar~ el puerto 🫦");
    println!("servidor iniciado en ip ws://{} ", ip);

    loop {
        let (stream, address) = listener
            .accept()
            .await
            .expect("no se pudo aceptar la conexion");

        println!("cliente conectado: {}", address);

        tokio::spawn(async move {
            let websocket = accept_async(stream)
                .await
                .expect("websocket apreton de manos fallado");

            println!("conexion de websocket: {}", address);

            let (_, mut receiver) = websocket.split();

            while let Some(message) = receiver.next().await {
                match message {
                    Ok(message) => {
                        if message.is_text() {
                            println!("Received from {address}: {}", message.to_text().unwrap());
                        } else {
                            println!("Received non-text message from {address}");
                        }
                    }

                    Err(error) => {
                        eprintln!("WebSocket error from {address}: {error}");
                        break;
                    }
                }
            }

            println!("Client disconnected: {address}");
        });
    }
    println!("adios")
}
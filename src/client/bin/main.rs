mod cliente;
use cliente::Cliente;
#[tokio::main]
async fn main() {
    let c = Cliente::new();
    c.start("ws://127.0.0.1:8080").await;
}

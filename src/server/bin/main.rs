mod servidor;
use servidor::Servidor;

#[tokio::main]
async fn main() {
    let mut s: Servidor = Servidor::new();
    s.start("127.0.0.1:8080").await;
}
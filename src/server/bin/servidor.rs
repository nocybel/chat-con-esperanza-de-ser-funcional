use std::collections::HashMap;
use tokio::net::TcpListener;
mod usuario;
use usuario::Usuario;

pub struct Servidor<'a> {
    usuarios: HashMap<&'a str, Usuario<'a>>,
}

impl Servidor<'_> {
    pub fn new() -> Self {
        println!("Servidor :D");
        
        Servidor {
            usuarios: HashMap::new(),
        }
    }

    pub async fn start(&mut self, ip: &str) {
        let listener = TcpListener::bind(ip).await.unwrap();
        println!("websocket en {ip}");

        loop {
            self.usuarios.insert("hola", Usuario::new(&listener).await);
        }
    }
}


use std::collections::HashMap;
use tokio::sync::mpsc;

mod usuario;
use usuario::Usuario;
mod sala;
use sala::Sala;

struct Servidor<'a> {
    usuarios: HashMap<String, Usuario<'a>>,
    salas: HashMap<String, Sala<'a>>,
    //rx: 
}

impl<'a> Servidor<'a> {
    
}
use std::collections::HashMap;

mod usuario;
use usuario::Usuario;
mod sala;
use sala::Sala;

struct Servidor<'a> {
    usuarios: HashMap<&'a str, Usuario<'a>>,
    salas: HashMap<&'a str, Sala<'a>>,
    //websocket coso ns
}

impl<'a> Servidor<'a> {
    pub fn new() -> Self {
        Servidor {
            usuarios: HashMap::new(),
            salas: HashMap::new(),
        }
    }

    pub fn cmd_identify(&mut self, username: &'a str) {
        if !self.usuarios.contains_key(username) {
            self.usuarios.insert(username, usuario::Usuario::new(username));
            // usuario agregado, mandar respuesta apropiada
        }
        // ya existe usuario, mandar respuesta apropiada
    }

    pub fn cmd_status(&mut self, usuario: &'a str, status: &'a str) {
        if status == "ACTIVE" || status == "AWAY" || status == "BUSY" {
            let blegh = self.usuarios.get_mut(usuario).unwrap();
            if status != blegh.get_status() {
                blegh.set_status(status);
                // broadcast nuevo status
            }   
        }
    }

    pub fn cmd_users(&self, usuario: &'a str) {
        let hola = "hola"; // falta formatear el json con el diccionario de usuarios
        self.usuarios.get(usuario).unwrap().envia_mensaje(hola);
    }

    fn get_usuarios(&self) -> Vec<&str> {
        let mut v: Vec<&str> = Vec::new();
        for u in &self.usuarios {
            v.push(u.0);
        }
        return v;
    }
}
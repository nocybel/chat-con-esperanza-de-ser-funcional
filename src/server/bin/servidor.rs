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
        let usuarios = self.get_usuarios();
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

    pub fn cmd_text(&self, mensaje: &str, username: &str) {
        //enviar mensaje a username :p
    }

    pub fn cmd_public_text(&self, mensaje: &str) {
        for username in self.get_usuarios() {
            //enviar mensaje a todos
        }
    }

    pub fn cmd_new_room(&mut self, nombre: &'a str, usuario: &'a str) {
        if !self.salas.contains_key(nombre) {
            self.salas.insert(nombre, sala::Sala::new(usuario, nombre.to_string()));
            // usuario agregado, mandar respuesta apropiada
        }
        // ya existe sala, mandar respuesta apropiada
    }

    pub fn cmd_invite(&mut self, sala: &'a str, from: &'a str, to: Vec<&'a str>) {
        if !self.salas.contains_key(sala) {
            // no existe sala
        }

        if !self.salas.get(sala).unwrap().tiene_miembro(from) {
            // usuario no es miembro
        }

        for sujeto in to {
            if self.usuarios.contains_key(sujeto) {
                self.salas.get_mut(sala).unwrap().invitar(sujeto);
                // notificar al sujeto que fue invitado
            }
            // no existe sujeto
        }
    }
}

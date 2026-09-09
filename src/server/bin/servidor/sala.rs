use crate::servidor::usuario::Usuario;

pub struct Sala<'a> {
    nombre: String,
    miembros: Vec<&'a str>,
    invitados: Vec<&'a str>,
}

impl<'a> Sala<'a> {
    // Constructor por parámetros
    pub fn new(creador: &'a str, nombre: String) -> Self {
        let mut sala = Sala {
            nombre: nombre,
            miembros: Vec::new(),
            invitados: Vec::new(),
        };
        sala.miembros.push(creador);
        
        return sala;
    }

    // Método para recibir lista de miembros en forma de vector de str
    pub fn get_miembros(&self) -> Vec<&str> {
        let mut sujetos: Vec<&str> = Vec::new();
        for usuario in &self.miembros {
            sujetos.push(usuario);
        }
        return sujetos;
    }

    pub fn tiene_miembro(&self, miembro: &'a str) -> bool {
        return self.miembros.contains(&miembro);
    }

    // Agrega un usuario a la lista de invitados
    pub fn invitar(&mut self, usuario: &'a str) {
        if !(self.invitados.contains(&usuario) || self.miembros.contains(&usuario)) {
            self.invitados.push(usuario);
        }
    }

    // Si un usuario está en la lista de invitados, lo agrega a la lista de miembros. Además, devuelve un número del 0 al 2 para representar el resultado
    fn unirse_a_sala(&mut self, usuario: &'a str) -> u8 {
        if self.miembros.contains(&usuario) {
            return 0; // Ya estaba en la sala
        }
        if self.invitados.contains(&usuario) {
            self.invitados.retain(|&x| x != usuario);
            self.miembros.push(usuario);
            return 1; // Se unio exitosamente
        }
        return 2; // No fue invitado
    }

    // Elimina a un usuario de la lista de miembros
    pub fn salir_de_sala(&mut self, usuario: &'a str) {
        self.miembros.retain(|&x| x != usuario);
    }

    // Te dice si ya no hay miembros o gente invitada
    pub fn es_vacia(&self) -> bool {
        return self.miembros.is_empty() && self.invitados.is_empty();
    }
}
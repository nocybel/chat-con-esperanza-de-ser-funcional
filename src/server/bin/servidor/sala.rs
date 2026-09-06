use crate::servidor::usuario::Usuario;

pub struct Sala<'a> {
    miembros: Vec<Usuario<'a>>,
    invitados: Vec<Usuario<'a>>,
}

impl<'a> Sala<'a> {
    fn new() -> Self {
        Sala {
            miembros: Vec::new(),
            invitados: Vec::new(),
        }
    }

    pub fn invitar(&mut self, usuario: Usuario<'a>) {
        if !self.invitados.contains(&usuario) {
            self.invitados.push(usuario);
        }
    }

    pub fn agregar(&mut self, usuario: Usuario<'a>) {
        if self.invitados.contains(&usuario) {
            self.invitados.retain(|&x| x != usuario);
            self.miembros.push(usuario);
        }
    }

    pub fn eliminar(&mut self, usuario: Usuario<'a>) {
        self.miembros.retain(|&x| x != usuario);
    }

    pub fn es_vacia(&self) -> bool {
        return self.miembros.is_empty() && self.invitados.is_empty();
    }

}
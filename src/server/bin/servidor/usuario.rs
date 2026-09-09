//#[derive(Copy, Clone)]
pub struct Usuario<'a> {
    nombre: &'a str,
    status: &'a str, // ACTIVE, AWAY, BUSY
    // canal de comunicación aquí
}

impl<'a> Usuario<'a> {
    pub fn new(nombre: &'a str) -> Self {
        Usuario {
            nombre: nombre,
            status: "ACTIVE",
        }
    }

    pub fn envia_mensaje(&self, mensaje: &'a str) {
        // envia un mensaje por el canal de comunicación
        println!("hola");
    }

    // regresa el nombre de usuario
    pub fn get_nombre(&self) -> &str {
        return self.nombre;
    }

    pub fn get_status(&self) -> &str {
        return self.status;
    }

    pub fn set_status(&mut self, status: &'a str) {
        self.status = status;
    }

    
}

impl<'a> PartialEq for Usuario<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.nombre == other.nombre;
    }
}
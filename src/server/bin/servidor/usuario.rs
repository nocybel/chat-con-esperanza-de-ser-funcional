#[derive(Copy, Clone)]
pub struct Usuario<'a> {
    nombre: &'a str,

}

impl<'a> Usuario<'a> {
    
}

impl<'a> PartialEq for Usuario<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.nombre == other.nombre;
    }
}
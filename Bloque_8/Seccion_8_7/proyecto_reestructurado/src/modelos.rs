#[derive(Debug, Clone)]
pub struct Producto {
    pub nombre: String,
    pub precio: f64,
    pub disponible: bool,
}

impl Producto {
    pub fn nuevo(nombre: &str, precio: f64, disponible: bool) -> Self {
        Self {
            nombre: String::from(nombre),
            precio,
            disponible,
        }
    }
}
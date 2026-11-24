use crate::modelos::Producto;

pub fn filtrar_disponibles(productos: &[Producto]) -> Vec<&Producto> {
    productos
        .iter()
        .filter(|producto| producto.disponible)
        .collect()
}

pub fn calcular_total(productos: &[&Producto]) -> f64 {
    productos.iter().map(|producto| producto.precio).sum()
}
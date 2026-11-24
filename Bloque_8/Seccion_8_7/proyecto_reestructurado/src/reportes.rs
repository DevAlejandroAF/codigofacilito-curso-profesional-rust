use crate::modelos::Producto;

pub fn imprimir_productos(titulo: &str, productos: &[&Producto]) {
    println!("{titulo}");

    for producto in productos {
        println!("- {}: ${:.2}", producto.nombre, producto.precio);
    }
}
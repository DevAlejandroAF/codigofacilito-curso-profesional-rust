/*
Sección 4.7 - Ejemplo práctico

Descripción:
Este programa diseña funciones seguras que evitan copias innecesarias.
Se usan referencias, slices, borrowing mutable y retornos prestados.

Objetivo del programa:
1. Guardar productos en un Vec.
2. Leer productos usando slices y referencias.
3. Modificar un producto usando una referencia mutable.
4. Buscar un producto y regresar una referencia, no una copia.
5. Calcular totales sin tomar ownership de la colección.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p ejemplo_practico_memoria
*/

struct Producto {
    nombre: String,
    precio: f64,
    etiquetas: Vec<String>,
}

fn imprimir_catalogo(productos: &[Producto]) {
    println!("Catálogo:");

    for producto in productos {
        println!("- {}: ${:.2}", producto.nombre, producto.precio);
    }
}

fn calcular_total(productos: &[Producto]) -> f64 {
    let mut total = 0.0;

    for producto in productos {
        total += producto.precio;
    }

    total
}

fn buscar_producto<'a>(productos: &'a [Producto], nombre: &str) -> Option<&'a Producto> {
    for producto in productos {
        if producto.nombre == nombre {
            return Some(producto);
        }
    }

    None
}

fn agregar_etiqueta(producto: &mut Producto, etiqueta: &str) {
    producto.etiquetas.push(String::from(etiqueta));
}

fn main() {
    let mut productos = vec![
        Producto {
            nombre: String::from("Teclado"),
            precio: 850.0,
            etiquetas: vec![String::from("periférico")],
        },
        Producto {
            nombre: String::from("Monitor"),
            precio: 4200.0,
            etiquetas: vec![String::from("pantalla")],
        },
    ];

    imprimir_catalogo(&productos);

    let total = calcular_total(&productos);
    println!("Total del catálogo: ${total:.2}");

    agregar_etiqueta(&mut productos[0], "oferta");

    match buscar_producto(&productos, "Teclado") {
        Some(producto) => {
            println!("Producto encontrado: {}", producto.nombre);
            println!("Etiquetas: {:?}", producto.etiquetas);
        }
        None => println!("Producto no encontrado"),
    }
}

/*
Ejercicio autónomo:
1. Agrega un producto más al vector productos.
2. Crea una función llamada aplicar_descuento que reciba &mut Producto y un porcentaje f64.
3. La función debe modificar el precio del producto sin regresar una copia.
4. Crea una función llamada productos_caros que reciba un slice de Producto y un precio mínimo.
5. Esa función debe imprimir los productos cuyo precio sea mayor o igual al mínimo.
6. Explica en un comentario cuándo conviene usar referencia, referencia mutable o ownership.
*/
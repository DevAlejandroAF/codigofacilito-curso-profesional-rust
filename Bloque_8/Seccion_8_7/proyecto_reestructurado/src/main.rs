use proyecto_reestructurado::modelos::Producto;
use proyecto_reestructurado::reportes::imprimir_productos;
use proyecto_reestructurado::servicios::{calcular_total, filtrar_disponibles};

fn main() {
    let productos = vec![
        Producto::nuevo("Teclado", 850.0, true),
        Producto::nuevo("Monitor", 4200.0, true),
        Producto::nuevo("Cable HDMI", 180.0, false),
    ];

    let disponibles = filtrar_disponibles(&productos);
    let total = calcular_total(&disponibles);

    imprimir_productos("Productos disponibles:", &disponibles);
    println!("Total disponible: ${total:.2}");
}

/*
Ejercicio autónomo:
1. Crea un nuevo módulo llamado validaciones.rs.
2. Dentro, agrega una función pública precio_valido que reciba f64 y regrese bool.
3. Declara el módulo desde lib.rs.
4. Usa precio_valido antes de crear o procesar productos.
5. Agrega una función en servicios.rs que cuente productos no disponibles.
6. Agrega un comentario explicando cómo mejoró la organización al separar modelos, servicios, reportes y validaciones.
*/
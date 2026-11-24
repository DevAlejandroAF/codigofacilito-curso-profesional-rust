/*
Sección 3.3 - Colecciones comunes

Descripción:
Este programa muestra el uso básico de Vec<T> y HashMap<K, V>.
También compara cuándo conviene usar un array y cuándo una colección dinámica.

Diagrama conceptual:
Array        -> tamaño fijo, todos los elementos del mismo tipo
Vec<T>       -> lista dinámica, puede crecer o reducirse
HashMap<K,V> -> asociación entre llaves y valores

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p colecciones_comunes
*/

use std::collections::HashMap;

fn main() {
    let calificaciones_fijas = [90, 85, 100];
    println!("Array fijo: {calificaciones_fijas:?}");

    let mut tareas: Vec<String> = Vec::new();
    tareas.push(String::from("Leer documentación"));
    tareas.push(String::from("Practicar con Cargo"));
    tareas.push(String::from("Resolver ejercicios"));

    println!("Lista de tareas:");
    for tarea in &tareas {
        println!("- {tarea}");
    }

    let primera_tarea = &tareas[0];
    println!("Primera tarea: {primera_tarea}");

    let mut puntajes: HashMap<String, u32> = HashMap::new();
    puntajes.insert(String::from("Ana"), 95);
    puntajes.insert(String::from("Luis"), 87);
    puntajes.insert(String::from("Sofía"), 100);

    let nombre = String::from("Ana");

    match puntajes.get(&nombre) {
        Some(puntaje) => println!("Puntaje de {nombre}: {puntaje}"),
        None => println!("No se encontró puntaje para {nombre}"),
    }

    for (estudiante, puntaje) in &puntajes {
        println!("{estudiante}: {puntaje}");
    }
}

/*
Ejercicio autónomo:
1. Crea un Vec<String> llamado compras.
2. Agrega al menos cuatro productos con push.
3. Recorre el vector con for e imprime cada producto.
4. Crea un HashMap<String, u32> para guardar el inventario de productos.
5. Inserta al menos tres productos con su cantidad disponible.
6. Busca un producto con get y muestra un mensaje usando match.
*/
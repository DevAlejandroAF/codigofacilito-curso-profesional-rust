/*
Sección 10.1 - Concurrencia segura en Rust

Descripción:
Este programa introduce la idea de concurrencia segura en Rust.
Usa thread::scope para ejecutar trabajo concurrente con referencias prestadas.

Diagrama conceptual:
Datos originales
├── slice izquierdo -> thread A
└── slice derecho   -> thread B

Idea clave:
Rust revisa ownership, borrowing y lifetimes también en código concurrente.
Eso ayuda a evitar accesos inválidos a memoria y condiciones de carrera en tiempo de compilación.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p concurrencia_segura
*/

use std::thread;

fn sumar_parte(nombre: &str, datos: &[i32]) -> i32 {
    let suma: i32 = datos.iter().sum();
    println!("{nombre} procesó {datos:?} y obtuvo {suma}");
    suma
}

fn main() {
    let datos = vec![10, 20, 30, 40, 50, 60];
    let mitad = datos.len() / 2;
    let (izquierda, derecha) = datos.split_at(mitad);

    thread::scope(|scope| {
        let tarea_izquierda = scope.spawn(|| sumar_parte("Thread A", izquierda));
        let tarea_derecha = scope.spawn(|| sumar_parte("Thread B", derecha));

        let total_izquierda = tarea_izquierda.join().expect("Thread A no debería fallar.");
        let total_derecha = tarea_derecha.join().expect("Thread B no debería fallar.");

        println!("Suma total concurrente: {}", total_izquierda + total_derecha);
    });

    println!("Datos originales disponibles después del scope: {datos:?}");
}

/*
Ejercicio autónomo:
1. Crea un vector con ocho números enteros.
2. Divide el vector en dos slices usando split_at.
3. Usa thread::scope para procesar ambos slices en threads separados.
4. En cada thread, calcula el valor máximo del slice correspondiente.
5. Une los resultados con join e imprime el mayor de ambos.
6. Agrega un comentario explicando por qué thread::scope permite usar referencias de forma segura.
*/
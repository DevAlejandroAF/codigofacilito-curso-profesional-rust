/*
Sección 10.2 - Threads

Descripción:
Este programa muestra cómo crear threads con spawn, esperar su resultado con join
y repartir trabajo básico entre varios threads.

Diagrama conceptual:
main thread
├── spawn -> worker 1 -> subtotal
├── spawn -> worker 2 -> subtotal
└── join  -> combina resultados

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p threads
*/

use std::thread;

fn procesar_lote(id: usize, valores: Vec<i32>) -> i32 {
    println!("Worker {id} recibió {valores:?}");

    let subtotal: i32 = valores.iter().sum();

    println!("Worker {id} terminó con subtotal {subtotal}");
    subtotal
}

fn main() {
    let lotes = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut handles = Vec::new();

    for (indice, lote) in lotes.into_iter().enumerate() {
        let id = indice + 1;

        let handle = thread::spawn(move || procesar_lote(id, lote));
        handles.push(handle);
    }

    let mut total = 0;

    for handle in handles {
        let subtotal = handle.join().expect("El worker no debería fallar.");
        total += subtotal;
    }

    println!("Total calculado por todos los workers: {total}");
}

/*
Ejercicio autónomo:
1. Crea una colección de lotes, donde cada lote sea un Vec<i32>.
2. Crea un thread por cada lote usando thread::spawn.
3. Usa move para transferir cada lote al thread correspondiente.
4. Haz que cada thread calcule el producto de los números de su lote.
5. Usa join para recuperar cada resultado y calcular el producto total.
6. Agrega un comentario explicando por qué join es necesario antes de usar los resultados.
*/
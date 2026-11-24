/*
Sección 10.4 - Estado compartido

Descripción:
Este programa muestra cómo compartir estado mutable entre threads usando Arc y Mutex.
Varios threads incrementan un contador protegido por un candado.

Diagrama conceptual:
Arc<Mutex<i32>>
├── clone -> thread 1 -> lock -> modifica contador
├── clone -> thread 2 -> lock -> modifica contador
└── clone -> thread 3 -> lock -> modifica contador

Idea clave:
Arc permite múltiples dueños entre threads.
Mutex permite modificar el dato de forma sincronizada.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p estado_compartido
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for id in 1..=5 {
        let contador_compartido = Arc::clone(&contador);

        let handle = thread::spawn(move || {
            for _ in 0..3 {
                let mut valor = contador_compartido
                    .lock()
                    .expect("El mutex no debería estar contaminado.");

                *valor += 1;
                println!("Thread {id} incrementó el contador a {valor}");
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("El thread no debería fallar.");
    }

    let resultado = contador.lock().expect("El mutex debería poder bloquearse.");
    println!("Valor final del contador: {resultado}");
}

/*
Ejercicio autónomo:
1. Crea un Arc<Mutex<Vec<String>>> para guardar mensajes compartidos.
2. Lanza tres threads.
3. Cada thread debe bloquear el Mutex y agregar un mensaje al vector.
4. Espera todos los threads con join.
5. Imprime el vector final desde main.
6. Agrega un comentario explicando qué responsabilidad tiene Arc y qué responsabilidad tiene Mutex.
*/
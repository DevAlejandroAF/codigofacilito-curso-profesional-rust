/*
Sección 10.3 - Comunicación con canales

Descripción:
Este programa muestra cómo usar canales para enviar mensajes entre threads.
Se usa std::sync::mpsc para crear un transmisor y un receptor.

Diagrama conceptual:
worker 1 ─┐
worker 2 ─┼── tx.send(mensaje) ──> rx en main
worker 3 ─┘

Idea clave:
Un canal permite comunicar resultados sin compartir directamente la misma memoria.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p canales
*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 1..=3 {
        let tx_worker = tx.clone();

        thread::spawn(move || {
            let mensaje = format!("Worker {id} terminó su tarea.");
            thread::sleep(Duration::from_millis(200 * id));

            tx_worker
                .send(mensaje)
                .expect("El receptor debería seguir disponible.");
        });
    }

    drop(tx);

    for mensaje in rx {
        println!("Mensaje recibido: {mensaje}");
    }

    println!("Todos los mensajes fueron recibidos.");
}

/*
Ejercicio autónomo:
1. Crea un canal usando mpsc::channel.
2. Lanza cuatro threads.
3. Cada thread debe enviar un mensaje con su número de worker y un resultado numérico.
4. Cierra el transmisor original con drop(tx).
5. Recorre el receptor con for e imprime cada mensaje.
6. Agrega un comentario explicando por qué se clona el transmisor para cada thread.
*/
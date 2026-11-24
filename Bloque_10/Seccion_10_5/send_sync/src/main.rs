/*
Sección 10.5 - Send y Sync

Descripción:
Este programa introduce Send y Sync como traits relacionados con seguridad de concurrencia.
También muestra una función genérica que exige Send y otra que exige Sync.

Diagrama conceptual:
Send -> un valor puede moverse de un thread a otro.
Sync -> una referencia compartida a un valor puede usarse desde varios threads.

Idea clave:
Rust usa estos traits para impedir combinaciones inseguras en código concurrente.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p send_sync
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn requiere_send<T: Send>(valor: T) -> T {
    valor
}

fn requiere_sync<T: Sync>(_: &T) {
    println!("El tipo puede compartirse por referencia de forma segura.");
}

fn main() {
    let numero = requiere_send(10);
    println!("Número enviado de forma segura: {numero}");

    let texto = String::from("Rust verifica seguridad entre threads");
    let texto = requiere_send(texto);
    requiere_sync(&texto);

    let datos = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = Vec::new();

    for incremento in [10, 20, 30] {
        let datos_compartidos = Arc::clone(&datos);

        let handle = thread::spawn(move || {
            let mut valores = datos_compartidos
                .lock()
                .expect("El mutex debería poder bloquearse.");

            valores.push(incremento);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("El thread no debería fallar.");
    }

    println!("Datos finales: {:?}", datos.lock().expect("Mutex disponible"));
}

/*
Ejercicio autónomo:
1. Crea una función genérica llamada aceptar_send que reciba y regrese un valor T: Send.
2. Crea una función genérica llamada aceptar_sync que reciba una referencia a T: Sync.
3. Prueba ambas funciones con tipos simples como i32 y String.
4. Crea un Arc<Mutex<Vec<i32>>> y compártelo entre dos threads.
5. Modifica el vector desde ambos threads y muestra el resultado final.
6. Agrega un comentario explicando por qué Send y Sync ayudan a la seguridad de concurrencia.
*/
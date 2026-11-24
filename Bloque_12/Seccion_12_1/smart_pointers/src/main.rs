/*
Sección 12.1 - Smart pointers

Descripción:
Este programa muestra usos prácticos de Box<T>, Rc<T> y Arc<T>.
Box<T> coloca datos en el heap, Rc<T> permite ownership compartido en un solo thread
y Arc<T> permite ownership compartido entre threads.

Diagrama conceptual:
Box<T> -> un solo owner, dato en el heap
Rc<T>  -> múltiples owners en un mismo thread
Arc<T> -> múltiples owners entre threads usando conteo atómico

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p smart_pointers
*/

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
enum Lista {
    Nodo(i32, Box<Lista>),
    Fin,
}

fn longitud_lista(lista: &Lista) -> usize {
    match lista {
        Lista::Nodo(_, siguiente) => 1 + longitud_lista(siguiente),
        Lista::Fin => 0,
    }
}

#[derive(Debug)]
struct Configuracion {
    nombre_app: String,
    modo: String,
}

fn main() {
    let lista = Lista::Nodo(
        10,
        Box::new(Lista::Nodo(20, Box::new(Lista::Nodo(30, Box::new(Lista::Fin))))),
    );

    println!("Lista enlazada: {lista:?}");
    println!("Longitud de la lista: {}", longitud_lista(&lista));

    let configuracion = Rc::new(Configuracion {
        nombre_app: String::from("CursoRust"),
        modo: String::from("aprendizaje"),
    });

    let modulo_reportes = Rc::clone(&configuracion);
    let modulo_usuarios = Rc::clone(&configuracion);

    println!("Reportes usa: {}", modulo_reportes.nombre_app);
    println!("Usuarios usa modo: {}", modulo_usuarios.modo);
    println!("Referencias Rc activas: {}", Rc::strong_count(&configuracion));

    let mensaje_compartido = Arc::new(String::from("Procesamiento concurrente con Arc"));
    let mut handles = Vec::new();

    for id in 1..=3 {
        let mensaje = Arc::clone(&mensaje_compartido);

        let handle = thread::spawn(move || {
            println!("Thread {id} recibió: {mensaje}");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("El thread no debería fallar.");
    }
}

/*
Ejercicio autónomo:
1. Crea un enum Arbol con variantes Hoja(i32) y Rama(Box<Arbol>, Box<Arbol>).
2. Crea una función que cuente cuántos nodos tiene el árbol.
3. Crea un Rc<String> que represente una configuración compartida en un solo thread.
4. Clona el Rc dos veces e imprime el conteo de referencias fuertes.
5. Crea un Arc<String> y compártelo entre dos threads.
6. Agrega un comentario explicando cuándo usarías Box<T>, Rc<T> y Arc<T>.
*/
/*
Sección 12.2 - Introducción a macros

Descripción:
Este programa muestra macros comunes como println!, vec! y format!,
y define una macro declarativa sencilla con macro_rules!.

Diagrama conceptual:
macro_rules! nombre_macro {
    (patrón) => {
        código generado
    };
}

Idea clave:
Una macro genera código antes de que el programa termine de compilarse.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p macros_intro
*/

macro_rules! crear_mensaje {
    ($nombre:expr, $tema:expr) => {
        format!("Hola, {}. Hoy practicaremos {}.", $nombre, $tema)
    };
}

macro_rules! registrar_pasos {
    ($($paso:expr),+ $(,)?) => {{
        let mut contador = 0;

        $(
            contador += 1;
            println!("Paso {}: {}", contador, $paso);
        )+

        contador
    }};
}

fn main() {
    let temas = vec!["smart pointers", "macros", "unsafe"];
    println!("Temas del bloque: {temas:?}");

    let mensaje = crear_mensaje!("Ana", "macros en Rust");
    println!("{mensaje}");

    let total_pasos = registrar_pasos!(
        "Identificar repetición",
        "Crear una regla macro",
        "Usar la macro con distintos argumentos",
    );

    println!("Total de pasos registrados: {total_pasos}");
}

/*
Ejercicio autónomo:
1. Crea una macro llamada etiqueta que reciba una categoría y un mensaje.
2. La macro debe regresar un String con formato "[categoria] mensaje".
3. Usa la macro al menos tres veces con categorías distintas.
4. Crea otra macro que reciba una lista de números y los imprima uno por uno.
5. Permite una coma final opcional en la macro de la lista.
6. Agrega un comentario explicando qué diferencia hay entre una función y una macro.
*/
/*
Sección 8.4 - Uso de crates.io

Descripción:
Este programa usa la crate regex como ejemplo de integración de una librería externa.
También incluye criterios básicos para evaluar una crate antes de integrarla.

Checklist para evaluar una crate:
1. Revisar documentación y ejemplos.
2. Revisar fecha de actualización y actividad.
3. Revisar número de descargas y adopción.
4. Revisar licencia.
5. Revisar si realmente resuelve el problema sin agregar complejidad innecesaria.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p uso_crates_io
*/

use regex::Regex;

fn es_email_basico(texto: &str) -> bool {
    let patron = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$")
        .expect("El patrón de regex debería ser válido.");

    patron.is_match(texto)
}

fn main() {
    let correos = [
        "ana@example.com",
        "correo_invalido",
        "luis@universidad.edu",
    ];

    for correo in correos {
        if es_email_basico(correo) {
            println!("{correo} parece un correo válido.");
        } else {
            println!("{correo} no parece un correo válido.");
        }
    }
}

/*
Ejercicio autónomo:
1. Busca una crate en crates.io que te parezca útil para un proyecto pequeño.
2. Revisa su documentación, licencia y actividad reciente.
3. Agrega la dependencia a Cargo.toml.
4. Crea una función pequeña que use esa crate.
5. Prueba la función desde main.
6. Agrega un comentario explicando por qué elegiste esa crate y qué problema resuelve.
*/
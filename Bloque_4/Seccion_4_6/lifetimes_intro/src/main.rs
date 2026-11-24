/*
Sección 4.6 - Introducción a lifetimes

Descripción:
Este programa introduce lifetimes como anotaciones que describen relaciones
entre referencias. No hacen que los datos vivan más tiempo; ayudan al compilador
a comprobar que las referencias devueltas siguen siendo válidas.

Diagrama conceptual:
fn texto_mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str
                       |          |          |
                       |          |          +-- el retorno vive como la vida común válida
                       |          +------------- segunda referencia con lifetime 'a
                       +------------------------ primera referencia con lifetime 'a

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p lifetimes_intro
*/

struct Fragmento<'a> {
    texto: &'a str,
}

fn texto_mas_largo<'a>(primero: &'a str, segundo: &'a str) -> &'a str {
    if primero.len() >= segundo.len() {
        primero
    } else {
        segundo
    }
}

fn crear_fragmento<'a>(texto: &'a str) -> Fragmento<'a> {
    let primera_oracion = texto.split('.').next().unwrap_or(texto);

    Fragmento {
        texto: primera_oracion,
    }
}

fn main() {
    let descripcion = String::from("Rust evita referencias inválidas. También ayuda a diseñar APIs seguras.");
    let alternativa = "Ownership";

    let mayor = texto_mas_largo(descripcion.as_str(), alternativa);
    println!("Texto más largo: {mayor}");

    let fragmento = crear_fragmento(descripcion.as_str());
    println!("Primer fragmento: {}", fragmento.texto);
}

/*
Ejercicio autónomo:
1. Crea una función llamada elegir_mensaje que reciba dos &str y regrese el más corto.
2. Agrega una anotación de lifetime explícita para relacionar los parámetros con el retorno.
3. Crea un struct llamado Nota que guarde una referencia a str.
4. Crea una función que reciba &str y regrese una Nota.
5. Prueba ambas funciones desde main.
6. Agrega un comentario explicando que los lifetimes no alargan la vida de los datos.
*/
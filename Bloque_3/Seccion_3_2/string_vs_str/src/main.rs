/*
Sección 3.2 - String vs &str

Descripción:
Este programa muestra diferencias prácticas entre String y &str.
Se trabaja con texto UTF-8, creación de cadenas, actualización y lectura segura.

Diagrama conceptual:
&str   -> vista prestada de texto, normalmente inmutable
String -> texto propio, almacenado en heap, puede crecer y modificarse

Importante:
Rust almacena strings como UTF-8. Por eso, no se recomienda acceder a una cadena
por índice numérico como si cada carácter ocupara exactamente un byte.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p string_vs_str
*/

fn imprimir_saludo(nombre: &str) {
    println!("Hola, {nombre}");
}

fn contar_bytes_y_caracteres(texto: &str) {
    println!("Texto: {texto}");
    println!("Bytes: {}", texto.len());
    println!("Caracteres Unicode: {}", texto.chars().count());
}

fn main() {
    let lenguaje: &str = "Rust";
    imprimir_saludo(lenguaje);

    let mut mensaje = String::from("Aprender");
    mensaje.push(' ');
    mensaje.push_str("Rust");
    mensaje.push_str(" es útil");

    println!("Mensaje completo: {mensaje}");

    let palabra = "México";
    contar_bytes_y_caracteres(palabra);

    let primeras_letras: String = palabra.chars().take(3).collect();
    println!("Primeras tres letras: {primeras_letras}");

    let mensaje_como_str: &str = &mensaje;
    imprimir_saludo(mensaje_como_str);
}

/*
Ejercicio autónomo:
1. Crea una variable &str con el nombre de tu lenguaje favorito.
2. Crea un String vacío usando String::new().
3. Agrega texto al String usando push_str y push.
4. Crea una función que reciba &str e imprima cuántos bytes y caracteres tiene.
5. Prueba la función con una palabra que tenga acentos o caracteres no ASCII.
6. Explica en un comentario por qué bytes y caracteres pueden no coincidir.
*/
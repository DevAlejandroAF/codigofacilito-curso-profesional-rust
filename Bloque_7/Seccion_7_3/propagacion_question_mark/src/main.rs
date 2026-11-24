/*
Sección 7.3 - Propagación de errores con ?

Descripción:
Este programa muestra cómo usar el operador ? para propagar errores.
Se evita anidar varios match cuando una función puede regresar Result.

Diagrama conceptual:
operacion()? 
    Ok(valor)  -> extrae el valor y continúa
    Err(error) -> regresa el error desde la función actual

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p propagacion_question_mark
*/

use std::error::Error;
use std::fs;

fn leer_archivo(ruta: &str) -> Result<String, std::io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    Ok(contenido)
}

fn leer_numero_desde_archivo(ruta: &str) -> Result<i32, Box<dyn Error>> {
    let contenido = leer_archivo(ruta)?;
    let numero = contenido.trim().parse::<i32>()?;
    Ok(numero)
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::write("numero.txt", "128")?;

    let numero = leer_numero_desde_archivo("numero.txt")?;
    println!("Número leído desde archivo: {numero}");

    Ok(())
}

/*
Ejercicio autónomo:
1. Crea una función llamada leer_texto que reciba una ruta y regrese Result<String, std::io::Error>.
2. Usa fs::read_to_string con el operador ? dentro de la función.
3. Crea una función llamada leer_decimal que lea el archivo y convierta el contenido a f64.
4. Haz que leer_decimal regrese Result<f64, Box<dyn std::error::Error>>.
5. Desde main, escribe un archivo temporal con un número decimal y léelo usando ?.
6. Agrega un comentario explicando cómo ? evita anidar varios match.
*/
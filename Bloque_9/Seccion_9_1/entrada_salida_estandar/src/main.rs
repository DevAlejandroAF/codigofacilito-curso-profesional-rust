/*
Sección 9.1 - Entrada y salida estándar

Descripción:
Este programa muestra cómo leer entrada interactiva desde stdin,
cómo escribir en stdout y stderr, y cómo presentar salida formateada.

Diagrama conceptual:
stdin  -> entrada estándar: datos que el programa lee
stdout -> salida estándar: resultados normales del programa
stderr -> salida de error: mensajes de error o advertencias

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p entrada_salida_estandar
*/

use std::io::{self, Write};

fn leer_linea(mensaje: &str) -> io::Result<String> {
    print!("{mensaje}");
    io::stdout().flush()?;

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada)?;

    Ok(entrada.trim().to_string())
}

fn main() -> io::Result<()> {
    let nombre = leer_linea("Escribe tu nombre: ")?;
    let edad_texto = leer_linea("Escribe tu edad: ")?;

    let edad = match edad_texto.parse::<u8>() {
        Ok(valor) => valor,
        Err(_) => {
            eprintln!("La edad ingresada no es válida. Se usará 0 como valor temporal.");
            0
        }
    };

    println!("Resumen del registro");
    println!("Nombre: {nombre}");
    println!("Edad: {edad}");
    println!("Mensaje formateado: {nombre} tiene {edad} años.");

    Ok(())
}

/*
Ejercicio autónomo:
1. Crea una función llamada leer_dato que reciba un mensaje y regrese io::Result<String>.
2. Pide al usuario su ciudad y su lenguaje de programación favorito.
3. Limpia la entrada usando trim.
4. Imprime un resumen con formato claro.
5. Usa eprintln! para mostrar una advertencia si algún dato queda vacío.
6. Agrega un comentario explicando la diferencia entre stdout y stderr.
*/
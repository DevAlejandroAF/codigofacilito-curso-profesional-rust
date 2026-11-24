/*
Sección 9.4 - Configuración y variables de entorno

Descripción:
Este programa separa entrada del usuario, argumentos de línea de comandos
y configuración obtenida desde variables de entorno.

Diagrama conceptual:
Argumentos CLI        -> valores que cambian en cada ejecución
Variables de entorno  -> configuración externa del programa
Entrada interactiva   -> datos escritos por el usuario durante la ejecución

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
MODO_APP=debug cargo run -p configuracion_entorno -- Maeva
cargo run -p configuracion_entorno -- Ana
*/

use std::env;
use std::io::{self, Write};

#[derive(Debug)]
struct Config {
    modo: String,
    nombre_usuario: String,
}

impl Config {
    fn desde_entorno_y_args(args: &[String]) -> Self {
        let modo = env::var("MODO_APP").unwrap_or_else(|_| String::from("produccion"));
        let nombre_usuario = args
            .get(1)
            .cloned()
            .unwrap_or_else(|| String::from("usuario"));

        Self { modo, nombre_usuario }
    }
}

fn confirmar_ejecucion() -> io::Result<bool> {
    print!("¿Deseas continuar? (s/n): ");
    io::stdout().flush()?;

    let mut respuesta = String::new();
    io::stdin().read_line(&mut respuesta)?;

    Ok(respuesta.trim().eq_ignore_ascii_case("s"))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::desde_entorno_y_args(&args);

    println!("Configuración cargada: {config:?}");

    if confirmar_ejecucion()? {
        println!("Ejecutando en modo '{}' para {}.", config.modo, config.nombre_usuario);
    } else {
        println!("Ejecución cancelada por el usuario.");
    }

    Ok(())
}

/*
Ejercicio autónomo:
1. Agrega una variable de entorno llamada IDIOMA_APP.
2. Si no existe, usa "es" como valor por defecto.
3. Agrega ese campo al struct Config.
4. Permite recibir el nombre del usuario por argumento de línea de comandos.
5. Mantén la confirmación interactiva separada de la configuración.
6. Agrega un comentario explicando por qué conviene separar configuración de entrada interactiva.
*/
/*
Sección 9.2 - Argumentos de línea de comandos

Descripción:
Este programa usa std::env::args para leer argumentos desde la terminal
y realiza un parsing básico de parámetros.

Diagrama conceptual:
cargo run -p argumentos_cli -- buscar rust archivo.txt
                                  |     |        |
                                  |     |        +-- ruta
                                  |     +----------- consulta
                                  +----------------- comando

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p argumentos_cli -- buscar rust notas.txt
cargo run -p argumentos_cli -- ayuda
*/

use std::env;

#[derive(Debug)]
enum Comando {
    Buscar { consulta: String, ruta: String },
    Ayuda,
}

fn parsear_argumentos(args: &[String]) -> Result<Comando, String> {
    if args.len() < 2 {
        return Ok(Comando::Ayuda);
    }

    match args[1].as_str() {
        "buscar" => {
            if args.len() != 4 {
                Err(String::from("Uso esperado: buscar <consulta> <ruta>"))
            } else {
                Ok(Comando::Buscar {
                    consulta: args[2].clone(),
                    ruta: args[3].clone(),
                })
            }
        }
        "ayuda" | "--help" | "-h" => Ok(Comando::Ayuda),
        otro => Err(format!("Comando desconocido: {otro}")),
    }
}

fn imprimir_ayuda() {
    println!("Uso:");
    println!("  argumentos_cli buscar <consulta> <ruta>");
    println!("  argumentos_cli ayuda");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match parsear_argumentos(&args) {
        Ok(Comando::Buscar { consulta, ruta }) => {
            println!("Buscando '{consulta}' en el archivo '{ruta}'.");
        }
        Ok(Comando::Ayuda) => imprimir_ayuda(),
        Err(error) => {
            eprintln!("Error: {error}");
            imprimir_ayuda();
        }
    }
}

/*
Ejercicio autónomo:
1. Agrega un comando llamado contar que reciba una ruta.
2. Representa el comando con una nueva variante del enum Comando.
3. Actualiza parsear_argumentos para reconocer contar <ruta>.
4. Actualiza imprimir_ayuda para documentar el nuevo comando.
5. En main, imprime un mensaje indicando qué archivo se contaría.
6. Agrega un comentario explicando por qué args[0] no debe usarse como dato de negocio.
*/
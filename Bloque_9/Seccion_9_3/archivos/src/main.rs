/*
Sección 9.3 - Lectura y escritura de archivos

Descripción:
Este programa usa std::fs para escribir, leer y validar archivos de texto.
También muestra manejo básico de rutas y errores comunes.

Diagrama conceptual:
fs::write          -> crea o reemplaza un archivo con contenido
fs::read_to_string -> lee todo el archivo como String
std::path::Path    -> representa rutas de archivos de forma portable

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p archivos
*/

use std::fs;
use std::io;
use std::path::Path;

fn guardar_reporte(ruta: &Path, contenido: &str) -> io::Result<()> {
    fs::write(ruta, contenido)
}

fn leer_reporte(ruta: &Path) -> io::Result<String> {
    fs::read_to_string(ruta)
}

fn contar_lineas(contenido: &str) -> usize {
    contenido.lines().count()
}

fn main() -> io::Result<()> {
    let ruta = Path::new("reporte_cli.txt");

    let contenido = "Reporte de ejemplo\nRust puede leer y escribir archivos\nFin del reporte";
    guardar_reporte(ruta, contenido)?;

    let texto = leer_reporte(ruta)?;
    let total_lineas = contar_lineas(&texto);

    println!("Contenido del archivo:");
    println!("{texto}");
    println!("Total de líneas: {total_lineas}");

    match leer_reporte(Path::new("archivo_inexistente.txt")) {
        Ok(texto) => println!("Archivo inesperadamente encontrado: {texto}"),
        Err(error) => eprintln!("No se pudo leer archivo_inexistente.txt: {error}"),
    }

    Ok(())
}

/*
Ejercicio autónomo:
1. Crea una función llamada guardar_lista que reciba una ruta y un slice de &str.
2. Une los elementos con saltos de línea y guarda el resultado con fs::write.
3. Crea una función llamada leer_lista que reciba una ruta y regrese io::Result<String>.
4. En main, guarda una lista de tareas en un archivo y luego léela.
5. Maneja con match el intento de leer un archivo que no existe.
6. Agrega un comentario explicando por qué las operaciones de archivos regresan Result.
*/
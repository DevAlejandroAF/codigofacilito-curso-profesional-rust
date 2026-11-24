/*
Sección 6.7 - Lifetimes a un nivel más profundo

Descripción:
Este programa profundiza en lifetimes combinándolos con structs, impl y funciones.
El objetivo es expresar relaciones entre referencias sin tomar ownership innecesario.

Diagrama conceptual:
struct Reporte<'a> {
    titulo: &'a str,
    contenido: &'a str,
}

impl<'a> Reporte<'a> {
    fn resumen(&self) -> &'a str { ... }
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p lifetimes_profundos
*/

struct Reporte<'a> {
    titulo: &'a str,
    contenido: &'a str,
}

impl<'a> Reporte<'a> {
    fn nuevo(titulo: &'a str, contenido: &'a str) -> Self {
        Self { titulo, contenido }
    }

    fn titulo(&self) -> &'a str {
        self.titulo
    }

    fn primer_parrafo(&self) -> &'a str {
        self.contenido.split("\n\n").next().unwrap_or(self.contenido)
    }
}

fn elegir_contenido_mas_largo<'a>(primero: &'a str, segundo: &'a str) -> &'a str {
    if primero.len() >= segundo.len() {
        primero
    } else {
        segundo
    }
}

fn main() {
    let titulo = String::from("Reporte de aprendizaje");
    let contenido = String::from("Rust permite expresar relaciones entre referencias.\n\nEso ayuda a evitar referencias inválidas.");

    let reporte = Reporte::nuevo(titulo.as_str(), contenido.as_str());

    println!("Título: {}", reporte.titulo());
    println!("Primer párrafo: {}", reporte.primer_parrafo());

    let alternativa = "Resumen breve";
    let elegido = elegir_contenido_mas_largo(reporte.primer_parrafo(), alternativa);

    println!("Contenido elegido: {elegido}");
}

/*
Ejercicio autónomo:
1. Crea un struct llamado Fragmento<'a> con un campo texto: &'a str.
2. Implementa una función asociada nuevo para Fragmento.
3. Implementa un método contenido que regrese &'a str.
4. Crea una función elegir_mas_corto que reciba dos &str con el mismo lifetime y regrese el más corto.
5. Prueba el struct y la función desde main.
6. Agrega un comentario explicando qué relación expresa el lifetime en tu código.
*/
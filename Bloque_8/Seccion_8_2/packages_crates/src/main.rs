use packages_crates::{contar_palabras, formatear_titulo};

fn main() {
    let titulo = formatear_titulo("Aplicación principal");
    let texto = "Rust organiza proyectos usando packages y crates";

    println!("{titulo}");
    println!("Texto: {texto}");
    println!("Cantidad de palabras: {}", contar_palabras(texto));
}
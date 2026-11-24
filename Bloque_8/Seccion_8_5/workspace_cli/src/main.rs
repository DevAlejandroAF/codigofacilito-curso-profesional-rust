use workspace_core::{crear_slug, normalizar_nombre};

fn main() {
    let nombre = "  Curso Avanzado de Rust  ";

    let normalizado = normalizar_nombre(nombre);
    let slug = crear_slug(nombre);

    println!("Nombre original: '{nombre}'");
    println!("Nombre normalizado: '{normalizado}'");
    println!("Slug: {slug}");
}
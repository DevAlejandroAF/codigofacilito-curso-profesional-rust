use packages_crates::formatear_titulo;

fn main() {
    let titulo = formatear_titulo("Reporte secundario");

    println!("{titulo}");
    println!("Este binario usa la misma librería del package.");
}
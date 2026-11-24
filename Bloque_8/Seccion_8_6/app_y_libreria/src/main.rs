use app_y_libreria::{calcular_promedio, estado_academico};

fn main() {
    let calificaciones = [88.0, 92.5, 76.0];

    match calcular_promedio(&calificaciones) {
        Some(promedio) => {
            println!("Promedio: {promedio:.2}");
            println!("Estado: {}", estado_academico(promedio));
        }
        None => println!("No hay calificaciones registradas."),
    }
}
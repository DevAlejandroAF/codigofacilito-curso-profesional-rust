/*
Sección 5.4 - Option<T> como caso de uso central

Descripción:
Este programa muestra Option<T> como una forma explícita de representar
la presencia o ausencia de un valor.

Diagrama conceptual:
Option<T>
    Some(valor) -> sí hay dato
    None        -> no hay dato

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p option_caso_central
*/

fn buscar_calificacion(nombre: &str) -> Option<u32> {
    if nombre == "Ana" {
        Some(95)
    } else if nombre == "Luis" {
        Some(88)
    } else {
        None
    }
}

fn main() {
    let resultado_ana = buscar_calificacion("Ana");
    let resultado_sofia = buscar_calificacion("Sofía");

    match resultado_ana {
        Some(calificacion) => println!("Ana tiene calificación de {calificacion}."),
        None => println!("Ana no tiene calificación registrada."),
    }

    match resultado_sofia {
        Some(calificacion) => println!("Sofía tiene calificación de {calificacion}."),
        None => println!("Sofía no tiene calificación registrada."),
    }
}

/*
Ejercicio autónomo:
1. Crea una función llamada buscar_descuento que reciba el nombre de un producto.
2. Haz que regrese Option<u32>.
3. Devuelve Some con un porcentaje para algunos productos y None para otros.
4. Desde main, llama la función al menos dos veces.
5. Usa match para imprimir si el descuento existe o no.
6. Agrega un comentario explicando por qué Option<T> es mejor que usar un valor “especial” inventado.
*/
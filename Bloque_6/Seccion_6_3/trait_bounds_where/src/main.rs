/*
Sección 6.3 - Trait bounds y cláusulas where

Descripción:
Este programa muestra cómo restringir tipos genéricos usando trait bounds.
También introduce cláusulas where para hacer más legibles las firmas complejas.

Diagrama conceptual:
fn funcion<T: Trait>(valor: T) { ... }

fn funcion<T, U>(a: T, b: U)
where
    T: TraitA,
    U: TraitB,
{
    ...
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p trait_bounds_where
*/

use std::fmt::Display;

fn imprimir_valor<T: Display>(valor: T) {
    println!("Valor recibido: {valor}");
}

fn comparar_e_imprimir<T>(a: T, b: T)
where
    T: Display + PartialOrd,
{
    if a >= b {
        println!("{a} es mayor o igual que {b}");
    } else {
        println!("{a} es menor que {b}");
    }
}

fn combinar_descripciones<T, U>(primero: T, segundo: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{primero} | {segundo}")
}

fn main() {
    imprimir_valor("Rust");
    imprimir_valor(2026);

    comparar_e_imprimir(10, 7);
    comparar_e_imprimir(3.5, 8.2);

    let descripcion = combinar_descripciones("Bloque", 6);
    println!("Descripción combinada: {descripcion}");
}

/*
Ejercicio autónomo:
1. Crea una función llamada mostrar_doble que reciba un valor genérico T.
2. Restringe T para que implemente Display y Copy.
3. La función debe imprimir el valor dos veces en el mismo mensaje.
4. Crea una función llamada elegir_mayor que reciba dos valores del mismo tipo.
5. Usa where para exigir que el tipo implemente PartialOrd y Copy.
6. Prueba ambas funciones desde main con números enteros y flotantes.
*/
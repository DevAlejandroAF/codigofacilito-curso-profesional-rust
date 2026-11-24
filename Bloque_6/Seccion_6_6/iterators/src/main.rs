/*
Sección 6.6 - Iterators

Descripción:
Este programa muestra cómo procesar colecciones usando iterators.
Se trabajan map, filter, collect, sum y diferencias básicas entre iter, iter_mut e into_iter.

Diagrama conceptual:
colección.iter()
    .filter(...)
    .map(...)
    .collect()

Idea clave:
Los iteradores son lazy: no hacen trabajo real hasta que se consumen.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p iterators
*/

fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6];

    let pares_al_cuadrado: Vec<i32> = numeros
        .iter()
        .filter(|numero| *numero % 2 == 0)
        .map(|numero| numero * numero)
        .collect();

    println!("Pares al cuadrado: {pares_al_cuadrado:?}");

    let suma_total: i32 = numeros.iter().sum();
    println!("Suma total: {suma_total}");

    let mut puntos = vec![10, 20, 30];

    for punto in puntos.iter_mut() {
        *punto += 5;
    }

    println!("Puntos actualizados: {puntos:?}");

    let nombres = vec![String::from("Ana"), String::from("Luis")];

    for nombre in nombres.into_iter() {
        println!("Nombre consumido: {nombre}");
    }
}

/*
Ejercicio autónomo:
1. Crea un vector con números del 1 al 10.
2. Usa iter, filter y collect para crear un Vec<i32> con los números impares.
3. Usa map para crear otro vector con esos números multiplicados por 3.
4. Usa sum para calcular la suma de los valores finales.
5. Usa iter_mut en un vector mutable para aumentar cada elemento en 1.
6. Agrega un comentario explicando por qué los iteradores son lazy.
*/
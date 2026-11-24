/*
Sección 6.5 - Closures

Descripción:
Este programa introduce closures como funciones anónimas que pueden guardarse,
pasarse como argumentos y capturar valores del entorno.

Diagrama conceptual:
let cierre = |parametro| expresion;

Diferencia general:
fn funcion(x: i32) -> i32 { x + 1 }     -> función nombrada
let cierre = |x: i32| x + 1;            -> closure guardada en variable

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p closures
*/

fn aplicar_operacion<F>(valor: i32, operacion: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operacion(valor)
}

fn main() {
    let sumar_uno = |numero: i32| numero + 1;
    let duplicar = |numero: i32| numero * 2;

    println!("5 + 1 = {}", sumar_uno(5));
    println!("5 * 2 = {}", duplicar(5));

    let resultado = aplicar_operacion(10, |numero| numero * numero);
    println!("10 al cuadrado = {resultado}");

    let factor = 3;
    let multiplicar_por_factor = |numero: i32| numero * factor;

    println!("7 por factor = {}", multiplicar_por_factor(7));

    let nombres = vec!["Ana", "Luis", "Sofía"];
    let nombres_con_a: Vec<&str> = nombres
        .into_iter()
        .filter(|nombre| nombre.contains('a') || nombre.contains('A'))
        .collect();

    println!("Nombres con la letra a: {nombres_con_a:?}");
}

/*
Ejercicio autónomo:
1. Crea una closure llamada restar_dos que reciba un i32 y le reste 2.
2. Crea una función llamada aplicar_a_dos_numeros que reciba dos i32 y una closure.
3. La closure debe recibir dos i32 y regresar un i32.
4. Usa la función para sumar dos números y para multiplicarlos.
5. Crea una closure que capture una variable externa llamada incremento.
6. Explica en un comentario qué significa que una closure capture su entorno.
*/
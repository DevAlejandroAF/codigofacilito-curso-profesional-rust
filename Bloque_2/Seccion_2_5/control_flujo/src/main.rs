/*
Sección 2.5 - Control de flujo

Descripción:
Este programa muestra cómo controlar el flujo de ejecución usando if, else if,
else, loop, while y for.

Diagrama conceptual:
if / else if / else -> tomar decisiones
loop                 -> repetir hasta usar break
while                -> repetir mientras una condición sea true
for                  -> recorrer rangos o colecciones

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p control_flujo
*/

fn main() {
    let calificacion = 86;

    if calificacion >= 90 {
        println!("Nivel: excelente");
    } else if calificacion >= 70 {
        println!("Nivel: aprobado");
    } else {
        println!("Nivel: no aprobado");
    }

    let tiene_beca = if calificacion >= 85 { true } else { false };
    println!("¿Tiene beca?: {tiene_beca}");

    let mut contador = 0;

    let resultado_loop = loop {
        contador += 1;

        if contador == 5 {
            break contador * 10;
        }
    };

    println!("Resultado del loop: {resultado_loop}");

    let mut cuenta_regresiva = 3;

    while cuenta_regresiva > 0 {
        println!("Cuenta regresiva: {cuenta_regresiva}");
        cuenta_regresiva -= 1;
    }

    println!("Inicio del recorrido con for:");

    let numeros = [10, 20, 30, 40];

    for numero in numeros {
        println!("Número del array: {numero}");
    }

    println!("Rango del 1 al 3:");

    for numero in 1..=3 {
        println!("Número del rango: {numero}");
    }
}

/*
Ejercicio autónomo:
1. Crea una variable llamada puntos con un valor entero.
2. Usa if, else if y else para imprimir si el nivel es "alto", "medio" o "bajo".
3. Usa un loop que incremente un contador hasta llegar a 4 y regrese el contador multiplicado por 2.
4. Usa while para hacer una cuenta regresiva desde 5.
5. Usa for para recorrer un array de tres nombres.
6. Imprime mensajes claros en cada parte del programa.
*/
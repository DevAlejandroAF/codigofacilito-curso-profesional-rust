/*
Sección 2.3 - Expresiones, sentencias y operadores

Descripción:
Este programa muestra la diferencia entre sentencias y expresiones,
así como el uso de operadores aritméticos, relacionales, lógicos y de asignación.

Diagrama conceptual:
Sentencia  -> ejecuta una acción, pero no produce un valor útil para asignar
Expresión  -> se evalúa y produce un valor

Ejemplos:
let edad = 22;       -> sentencia let
5 + 3                -> expresión
edad >= 18           -> expresión booleana
if condicion { ... } -> expresión si produce un valor compatible en cada rama

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p expresiones_operadores
*/

fn main() {
    let suma = 10 + 5;
    let resta = 10 - 5;
    let multiplicacion = 10 * 5;
    let division = 10 / 5;
    let residuo = 10 % 3;

    println!("Suma: {suma}");
    println!("Resta: {resta}");
    println!("Multiplicación: {multiplicacion}");
    println!("División: {division}");
    println!("Residuo: {residuo}");

    let edad = 20;
    let es_mayor_de_edad = edad >= 18;
    let tiene_identificacion = true;
    let puede_entrar = es_mayor_de_edad && tiene_identificacion;

    println!("¿Es mayor de edad?: {es_mayor_de_edad}");
    println!("¿Puede entrar?: {puede_entrar}");

    let mut puntos = 10;
    puntos += 5;
    puntos *= 2;

    println!("Puntos finales: {puntos}");

    let mensaje = {
        let puntos_minimos = 20;

        if puntos >= puntos_minimos {
            "Meta alcanzada"
        } else {
            "Meta pendiente"
        }
    };

    println!("Resultado: {mensaje}");
}

/*
Ejercicio autónomo:
1. Crea dos variables numéricas llamadas base y altura.
2. Calcula el área de un rectángulo usando base * altura.
3. Crea una variable booleana que indique si el área es mayor o igual a 50.
4. Usa operadores lógicos para combinar esa condición con otra condición inventada por ti.
5. Usa un bloque como expresión para guardar un mensaje en una variable.
6. Imprime el área y el mensaje final.
*/
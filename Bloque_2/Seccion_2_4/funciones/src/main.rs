/*
Sección 2.4 - Funciones

Descripción:
Este programa muestra cómo declarar funciones en Rust, cómo usar parámetros,
cómo regresar valores y cómo trabajar con expresiones implícitas.

Diagrama conceptual:
fn nombre_funcion(parametro: Tipo) -> TipoRetorno {
    expresión_final
}

Puntos clave:
- fn declara una función.
- Cada parámetro necesita tipo explícito.
- El tipo de retorno se escribe después de ->.
- La última expresión sin punto y coma puede ser el valor de retorno.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p funciones
*/

fn saludar(nombre: &str) {
    println!("Hola, {nombre}. Bienvenida al bloque de funciones.");
}

fn calcular_area_rectangulo(base: u32, altura: u32) -> u32 {
    base * altura
}

fn convertir_celsius_a_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn evaluar_aprobacion(promedio: f64) -> &'static str {
    if promedio >= 70.0 {
        "Aprobado"
    } else {
        "No aprobado"
    }
}

fn main() {
    saludar("Ana");

    let area = calcular_area_rectangulo(8, 5);
    println!("Área del rectángulo: {area}");

    let temperatura_f = convertir_celsius_a_fahrenheit(25.0);
    println!("25 grados Celsius equivalen a {temperatura_f} grados Fahrenheit");

    let resultado = evaluar_aprobacion(86.5);
    println!("Resultado del estudiante: {resultado}");
}

/*
Ejercicio autónomo:
1. Crea una función llamada calcular_doble que reciba un i32 y regrese el doble.
2. Crea una función llamada calcular_promedio que reciba tres calificaciones f64 y regrese su promedio.
3. Crea una función llamada obtener_estado que reciba un promedio f64 y regrese "Excelente", "Regular" o "Necesita mejorar".
4. Llama las tres funciones desde main.
5. Imprime los resultados de forma clara.
*/
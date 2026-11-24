/*
Sección 7.4 - Composición de resultados

Descripción:
Este programa muestra varias formas de trabajar con Option y Result:
match, unwrap, expect y combinadores como map, map_err, and_then y unwrap_or.

Diagrama conceptual:
match       -> manejo explícito de cada caso
unwrap      -> extrae el valor o hace panic si hay error
expect      -> extrae el valor o hace panic con mensaje personalizado
map         -> transforma el valor exitoso
and_then    -> encadena operaciones que también pueden fallar
unwrap_or   -> usa un valor por defecto si no hay éxito

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p composicion_resultados
*/

fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("División entre cero."))
    } else {
        Ok(a / b)
    }
}

fn convertir_y_dividir(texto: &str, divisor: i32) -> Result<i32, String> {
    texto
        .parse::<i32>()
        .map_err(|_| format!("'{texto}' no es un entero válido."))
        .and_then(|numero| dividir(numero, divisor))
}

fn main() {
    let resultado = dividir(100, 5);

    match resultado {
        Ok(valor) => println!("Resultado con match: {valor}"),
        Err(error) => println!("Error con match: {error}"),
    }

    let valor_seguro = dividir(50, 10).unwrap();
    println!("Valor extraído con unwrap en caso seguro: {valor_seguro}");

    let valor_con_mensaje = dividir(81, 9).expect("La división debería ser válida.");
    println!("Valor extraído con expect: {valor_con_mensaje}");

    let doble = dividir(40, 2).map(|valor| valor * 2);
    println!("Resultado transformado con map: {doble:?}");

    let intento_fallido = convertir_y_dividir("abc", 3).unwrap_or(-1);
    println!("Resultado con valor por defecto: {intento_fallido}");

    let encadenado = convertir_y_dividir("120", 6);
    println!("Resultado encadenado: {encadenado:?}");
}

/*
Ejercicio autónomo:
1. Crea una función llamada parsear_positivo que reciba &str y regrese Result<u32, String>.
2. Usa map_err para convertir el error de parseo en un mensaje propio.
3. Usa and_then para validar que el número sea mayor que cero.
4. Usa match para imprimir un caso exitoso.
5. Usa unwrap_or para dar un valor por defecto en un caso fallido.
6. Agrega un comentario explicando por qué unwrap y expect deben usarse con cuidado.
*/
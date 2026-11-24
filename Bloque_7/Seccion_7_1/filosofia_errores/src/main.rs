/*
Sección 7.1 - Filosofía de errores en Rust

Descripción:
Este programa muestra la diferencia entre errores recuperables y no recuperables.
También ilustra cuándo un Result permite continuar y cuándo panic! detiene el programa.

Diagrama conceptual:
Error recuperable     -> se modela con Result<T, E>
Error no recuperable  -> puede detener el programa con panic!

Idea clave:
Usa Result cuando quien llama puede decidir cómo manejar el problema.
Usa panic! cuando el programa llega a un estado que no debería ocurrir.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p filosofia_errores
*/

fn dividir(dividendo: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err(String::from("No se puede dividir entre cero."))
    } else {
        Ok(dividendo / divisor)
    }
}

fn obtener_elemento_seguro(valores: &[i32], indice: usize) -> Result<i32, String> {
    if indice < valores.len() {
        Ok(valores[indice])
    } else {
        Err(format!("El índice {indice} está fuera del rango."))
    }
}

fn validar_configuracion(valor: &str) {
    if valor.trim().is_empty() {
        panic!("La configuración no puede estar vacía.");
    }
}

fn main() {
    match dividir(20, 4) {
        Ok(resultado) => println!("Resultado de la división: {resultado}"),
        Err(error) => println!("Error recuperable: {error}"),
    }

    match dividir(20, 0) {
        Ok(resultado) => println!("Resultado de la división: {resultado}"),
        Err(error) => println!("Error recuperable: {error}"),
    }

    let numeros = [10, 20, 30];

    match obtener_elemento_seguro(&numeros, 5) {
        Ok(valor) => println!("Valor encontrado: {valor}"),
        Err(error) => println!("Error recuperable: {error}"),
    }

    validar_configuracion("modo_seguro");
    println!("La configuración pasó la validación.");
}

/*
Ejercicio autónomo:
1. Crea una función llamada calcular_porcentaje que reciba parte y total como f64.
2. La función debe regresar Result<f64, String>.
3. Si total es 0.0, regresa Err con un mensaje claro.
4. Si total es válido, regresa Ok con el porcentaje.
5. En main, prueba la función con un caso válido y uno inválido usando match.
6. Agrega un comentario explicando por qué este error debe ser recuperable y no un panic!.
*/
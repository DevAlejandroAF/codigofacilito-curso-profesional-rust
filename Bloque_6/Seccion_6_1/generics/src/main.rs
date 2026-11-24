/*
Sección 6.1 - Funciones y tipos genéricos

Descripción:
Este programa introduce funciones genéricas y structs genéricos.
El objetivo es reutilizar código para distintos tipos sin perder seguridad de tipos.

Diagrama conceptual:
fn identidad<T>(valor: T) -> T
            |
            +-- T representa un tipo concreto elegido al compilar

struct Par<T> {
    izquierdo: T,
    derecho: T,
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p generics
*/

#[derive(Debug)]
struct Par<T> {
    izquierdo: T,
    derecho: T,
}

fn identidad<T>(valor: T) -> T {
    valor
}

fn crear_par<T>(izquierdo: T, derecho: T) -> Par<T> {
    Par { izquierdo, derecho }
}

fn main() {
    let numero = identidad(42);
    let texto = identidad(String::from("Rust genérico"));

    println!("Número: {numero}");
    println!("Texto: {texto}");

    let par_numerico = crear_par(10, 20);
    let par_texto = crear_par("hola", "mundo");

    println!("Par numérico: {par_numerico:?}");
    println!("Par de texto: {par_texto:?}");
}

/*
Ejercicio autónomo:
1. Crea una función genérica llamada envolver_en_vector que reciba un valor T.
2. La función debe regresar un Vec<T> con ese único valor dentro.
3. Crea un struct genérico llamado Caja<T> con un campo contenido.
4. Crea una instancia de Caja<i32> y otra de Caja<String>.
5. Imprime los resultados usando Debug cuando sea necesario.
6. Agrega un comentario explicando qué representa T en una función o struct genérico.
*/
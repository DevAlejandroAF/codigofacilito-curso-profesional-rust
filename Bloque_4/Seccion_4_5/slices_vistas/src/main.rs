/*
Sección 4.5 - Slices y vistas prestadas de datos

Descripción:
Este programa muestra slices como vistas prestadas de strings y arrays.
También enseña cómo regresar partes de una cadena sin copiar el contenido.

Diagrama conceptual:
String completo:  "Rust seguro y rápido"
Slice de string:  &texto[0..4]  -> "Rust"

Array completo:   [5, 10, 15, 20, 25]
Slice de array:   &numeros[1..4] -> [10, 15, 20]

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p slices_vistas
*/

fn primera_palabra(texto: &str) -> &str {
    for (indice, byte) in texto.bytes().enumerate() {
        if byte == b' ' {
            return &texto[..indice];
        }
    }

    texto
}

fn sumar_slice(numeros: &[i32]) -> i32 {
    let mut suma = 0;

    for numero in numeros {
        suma += numero;
    }

    suma
}

fn main() {
    let frase = String::from("Rust seguro y rápido");
    let palabra = primera_palabra(&frase);

    println!("Frase completa: {frase}");
    println!("Primera palabra: {palabra}");

    let numeros = [5, 10, 15, 20, 25];
    let parte = &numeros[1..4];

    println!("Slice de números: {parte:?}");
    println!("Suma del slice: {}", sumar_slice(parte));

    let inicio = &frase[..4];
    println!("Slice inicial de la frase: {inicio}");
}

/*
Ejercicio autónomo:
1. Crea un String con una frase de al menos tres palabras.
2. Usa la función primera_palabra para obtener la primera palabra.
3. Crea un array de seis enteros.
4. Crea un slice con los elementos centrales del array.
5. Usa sumar_slice para sumar solo esa parte.
6. Explica en un comentario por qué el slice no toma ownership de los datos.
*/
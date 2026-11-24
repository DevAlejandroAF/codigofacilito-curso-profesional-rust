/*
Sección 5.5 - Pattern matching con match

Descripción:
Este programa muestra cómo usar match para destructurar valores,
cubrir casos exhaustivos y escribir lógica clara con patrones.

Diagrama conceptual:
match valor {
    patron_1 => ...,
    patron_2 => ...,
    _ => ...,
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p pattern_matching_match
*/

enum Comando {
    Saludar(String),
    Sumar(i32, i32),
    Salir,
}

fn ejecutar(comando: Comando) {
    match comando {
        Comando::Saludar(nombre) => {
            println!("Hola, {nombre}");
        }
        Comando::Sumar(a, b) => {
            println!("Resultado de la suma: {}", a + b);
        }
        Comando::Salir => {
            println!("Finalizando ejecución.");
        }
    }
}

fn main() {
    let accion_1 = Comando::Saludar(String::from("Maeva"));
    let accion_2 = Comando::Sumar(8, 12);
    let accion_3 = Comando::Salir;

    ejecutar(accion_1);
    ejecutar(accion_2);
    ejecutar(accion_3);
}

/*
Ejercicio autónomo:
1. Crea un enum llamado Mensaje con las variantes Texto(String), Numero(i32) y Vacio.
2. Crea una función que reciba el enum y use match para imprimir un mensaje apropiado.
3. Desestructura el String y el i32 dentro de las ramas correspondientes.
4. Incluye una rama para Vacio.
5. Crea al menos una instancia de cada variante en main.
6. Agrega un comentario explicando qué significa que match sea exhaustivo.
*/
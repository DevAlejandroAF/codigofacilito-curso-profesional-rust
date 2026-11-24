/*
Sección 4.4 - Borrowing y referencias

Descripción:
Este programa muestra borrowing con referencias inmutables y mutables.
También ilustra las reglas de aliasing que Rust revisa en tiempo de compilación.

Diagrama conceptual:
&T      -> referencia inmutable: permite leer sin tomar ownership.
&mut T  -> referencia mutable: permite modificar sin tomar ownership.

Regla clave:
Puedes tener muchas referencias inmutables o una sola referencia mutable,
pero no ambas al mismo tiempo sobre el mismo valor.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p borrowing_referencias
*/

fn calcular_longitud(texto: &String) -> usize {
    texto.len()
}

fn agregar_sufijo(texto: &mut String, sufijo: &str) {
    texto.push_str(sufijo);
}

fn imprimir_referencia(etiqueta: &str, texto: &String) {
    println!("{etiqueta}: {texto}");
}

fn main() {
    let mut mensaje = String::from("Aprender Rust");

    {
        let referencia_1 = &mensaje;
        let referencia_2 = &mensaje;

        imprimir_referencia("Referencia 1", referencia_1);
        imprimir_referencia("Referencia 2", referencia_2);
        println!("Longitud: {}", calcular_longitud(referencia_1));
    }

    {
        let referencia_mutable = &mut mensaje;
        agregar_sufijo(referencia_mutable, " mejora el diseño de software");
    }

    println!("Mensaje final: {mensaje}");
}

/*
Ejercicio autónomo:
1. Crea un String mutable llamado descripcion.
2. Crea dos referencias inmutables dentro de un bloque e imprímelas.
3. Después del bloque, crea una referencia mutable al mismo String.
4. Usa una función que reciba &mut String para agregar texto.
5. Imprime el resultado final.
6. Agrega un comentario explicando por qué las referencias inmutables deben terminar antes de usar la mutable.
*/
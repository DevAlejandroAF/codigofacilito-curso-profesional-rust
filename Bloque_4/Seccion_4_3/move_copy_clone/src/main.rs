/*
Sección 4.3 - Move, Copy y Clone

Descripción:
Este programa muestra la diferencia entre mover, copiar y clonar datos.
Se usan enteros, String y un struct simple que implementa Copy y Clone.

Diagrama conceptual:
Move  -> transfiere ownership; el valor anterior deja de poder usarse.
Copy  -> duplica implícitamente valores simples y baratos.
Clone -> duplica explícitamente, posiblemente copiando datos del heap.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p move_copy_clone
*/

#[derive(Debug, Copy, Clone)]
struct Punto {
    x: i32,
    y: i32,
}

fn imprimir_entero(valor: i32) {
    println!("Entero recibido: {valor}");
}

fn imprimir_texto(texto: String) {
    println!("Texto recibido: {texto}");
}

fn main() {
    let a = 10;
    let b = a;

    println!("a sigue disponible: {a}");
    println!("b contiene una copia: {b}");

    imprimir_entero(a);
    println!("a también sigue disponible después de la función: {a}");

    let texto_original = String::from("Rust");
    let texto_movido = texto_original;

    println!("Texto movido: {texto_movido}");

    let texto_base = String::from("Ownership");
    let texto_clonado = texto_base.clone();

    println!("Texto base: {texto_base}");
    println!("Texto clonado: {texto_clonado}");

    let punto_a = Punto { x: 3, y: 4 };
    let punto_b = punto_a;

    println!("Punto A: {punto_a:?}");
    println!("Punto B: {punto_b:?}");

    imprimir_texto(texto_clonado);
}

/*
Ejercicio autónomo:
1. Crea una variable i32 y asígnala a otra variable.
2. Imprime ambas variables para comprobar que el valor se copió.
3. Crea un String y muévelo a otra variable.
4. Agrega un comentario explicando por qué ya no se puede usar la variable original.
5. Crea otro String y usa clone para duplicarlo.
6. Imprime el String original y el clonado.
*/
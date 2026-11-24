/*
Sección 6.4 - Traits comunes del ecosistema estándar

Descripción:
Este programa muestra traits comunes como Debug, Clone, Copy, Default y PartialEq.
También enseña cómo derivarlos en tipos propios cuando sus campos lo permiten.

Diagrama conceptual:
Debug    -> permite imprimir con {:?}
Clone    -> duplicación explícita con .clone()
Copy     -> copia implícita para tipos simples
Default  -> valor por defecto con Default::default()
PartialEq -> comparación con == y !=

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p traits_estandar
*/

#[derive(Debug, Clone, PartialEq)]
struct Usuario {
    nombre: String,
    activo: bool,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let usuario_1 = Usuario {
        nombre: String::from("Ana"),
        activo: true,
    };

    let usuario_2 = usuario_1.clone();

    println!("Usuario 1: {usuario_1:?}");
    println!("Usuario 2 clonado: {usuario_2:?}");
    println!("¿Son iguales?: {}", usuario_1 == usuario_2);

    let punto_a = Punto { x: 3, y: 4 };
    let punto_b = punto_a;
    let punto_origen = Punto::default();

    println!("Punto A: {punto_a:?}");
    println!("Punto B copiado: {punto_b:?}");
    println!("Punto por defecto: {punto_origen:?}");
    println!("¿Punto A es el origen?: {}", punto_a == punto_origen);
}

/*
Ejercicio autónomo:
1. Crea un struct Producto con nombre: String y precio: f64.
2. Deriva Debug, Clone y PartialEq para Producto.
3. Crea un producto original y un producto clonado.
4. Imprime ambos con {:?} y compara si son iguales.
5. Crea un struct Coordenada con x: i32 y y: i32.
6. Deriva Debug, Copy, Clone, Default y PartialEq para Coordenada y prueba sus comportamientos.
*/
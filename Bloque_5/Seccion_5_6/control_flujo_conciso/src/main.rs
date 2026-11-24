/*
Sección 5.6 - Formas concisas de control de flujo

Descripción:
Este programa introduce if let, while let y let-else como formas
más concisas de trabajar con ciertos patrones comunes.

Diagrama conceptual:
if let patron = valor { ... }
while let patron = valor { ... }
let patron = valor else { ... };

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p control_flujo_conciso
*/

fn main() {
    let configuracion = Some(String::from("modo_oscuro"));

    if let Some(valor) = configuracion {
        println!("Configuración encontrada: {valor}");
    }

    let mut pila = vec![1, 2, 3];

    while let Some(numero) = pila.pop() {
        println!("Extrayendo de la pila: {numero}");
    }

    let opcion_usuario = Some("Rust");

    let Some(lenguaje) = opcion_usuario else {
        println!("No se recibió lenguaje.");
        return;
    };

    println!("Lenguaje seleccionado: {lenguaje}");
}

/*
Ejercicio autónomo:
1. Crea una variable Option<String> con un nombre de usuario.
2. Usa if let para imprimirlo si existe.
3. Crea un vector de enteros y usa while let con pop para vaciarlo.
4. Crea otra variable Option<&str>.
5. Usa let-else para extraer el valor o terminar la función si no existe.
6. Agrega un comentario explicando cuándo conviene usar estas formas en lugar de un match completo.
*/
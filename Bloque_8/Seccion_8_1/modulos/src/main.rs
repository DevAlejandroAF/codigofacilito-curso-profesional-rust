/*
Sección 8.1 - Organización con módulos

Descripción:
Este archivo es el punto de entrada del paquete. Declara módulos externos,
usa elementos públicos de esos módulos y muestra cómo se organiza una API pública.

Diagrama conceptual:
crate raíz: main.rs
├── usuarios.rs  -> define el tipo Usuario
└── reportes.rs  -> define funciones para reportes

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p modulos
*/

mod usuarios;
mod reportes;

use reportes::generar_resumen;
use usuarios::Usuario;

fn main() {
    let usuario = Usuario::nuevo("Ana", "ana@example.com");

    println!("Usuario registrado: {}", usuario.nombre());
    println!("{}", generar_resumen(&usuario));
}

/*
Ejercicio autónomo:
1. Crea un archivo nuevo llamado cursos.rs.
2. Declara ese módulo desde main.rs usando mod cursos.
3. Dentro de cursos.rs, crea un struct público Curso con un campo público nombre.
4. Crea una función pública llamada crear_curso_basico.
5. Usa el módulo cursos desde main.rs para crear e imprimir un curso.
6. Agrega un comentario explicando qué elementos tuvieron que ser pub y por qué.
*/
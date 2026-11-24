# Curso de Rust

Repositorio con el código, ejemplos y ejercicios del [Curso Profesional de Rust de Código Facilito](https://codigofacilito.com/cursos/rust-profesional/).

El contenido está organizado en bloques y secciones siguiendo la estructura del curso. Cada sección contiene ejemplos prácticos para reforzar los conceptos vistos durante las clases.

## Contenido

El curso está dividido en 13 bloques, comenzando con los fundamentos de Rust y avanzando hacia conceptos como ownership, manejo de errores, concurrencia, asincronía, pruebas, smart pointers y macros.

| Bloque                   | Temas principales                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------------- |
| [Bloque 1](./Bloque_1)   | Introducción a Rust, estructura de un proyecto y Hello World                                   |
| [Bloque 2](./Bloque_2)   | Variables y mutabilidad, tipos de datos, expresiones, operadores, funciones y control de flujo |
| [Bloque 3](./Bloque_3)   | Arrays, slices, tuplas, `String`, `str`, colecciones, métodos y estilo idiomático              |
| [Bloque 4](./Bloque_4)   | Stack y heap, ownership, move, copy, clone, borrowing, referencias, slices y lifetimes         |
| [Bloque 5](./Bloque_5)   | Structs, métodos, funciones asociadas, enums, `Option`, pattern matching y control de flujo    |
| [Bloque 6](./Bloque_6)   | Generics, traits, trait bounds, closures, iterators y lifetimes                                |
| [Bloque 7](./Bloque_7)   | Manejo de errores, `Option`, `Result`, operador `?`, composición y errores personalizados      |
| [Bloque 8](./Bloque_8)   | Módulos, packages, crates, dependencias con Cargo y organización de proyectos                  |
| [Bloque 9](./Bloque_9)   | Entrada y salida, argumentos CLI, archivos, variables de entorno y diseño de aplicaciones CLI  |
| [Bloque 10](./Bloque_10) | Concurrencia, threads, canales, estado compartido, `Send`, `Sync`, async/await y Tokio         |
| [Bloque 11](./Bloque_11) | Verificación continua, pruebas, documentación, formato y calidad de código                     |
| [Bloque 12](./Bloque_12) | Smart pointers, macros, `unsafe`, interoperabilidad y áreas de especialización                 |
| [Bloque 13](./Bloque_13) | Proyecto integrador                                                                            |

## Estructura del repositorio

Cada bloque contiene las secciones correspondientes a los temas vistos durante el curso.

```text
CursoRust/
├── Bloque_1/
├── Bloque_2/
├── Bloque_3/
├── Bloque_4/
├── Bloque_5/
├── Bloque_6/
├── Bloque_7/
├── Bloque_8/
├── Bloque_9/
├── Bloque_10/
├── Bloque_11/
├── Bloque_12/
└── Bloque_13/
```

La mayoría de las secciones contienen un proyecto independiente de Rust. Por ejemplo:

```text
Bloque_10/
├── Seccion_10_1/
│   └── concurrencia_segura/
├── Seccion_10_2/
│   └── threads/
├── Seccion_10_3/
│   └── canales/
├── Seccion_10_4/
│   └── estado_compartido/
├── Seccion_10_5/
│   └── send_sync/
├── Seccion_10_6/
│   └── async_await/
├── Seccion_10_7/
│   └── tokio_runtime/
└── Seccion_10_8/
    └── cliente_async/
```

## Requisitos

Para ejecutar los ejemplos necesitas tener instalados **Rust** y **Cargo**.

Puedes verificar tu instalación con:

```bash
rustc --version
cargo --version
```

Si todavía no tienes Rust instalado, puedes hacerlo utilizando `rustup` desde el [sitio oficial de Rust](https://www.rust-lang.org/tools/install).

## Cómo utilizar el repositorio

Clona el repositorio:

```bash
git clone https://github.com/MaeMazcort/CursoRust.git
```

Entra al directorio:

```bash
cd CursoRust
```

Navega al ejemplo que quieras ejecutar. Por ejemplo:

```bash
cd Bloque_10/Seccion_10_2/threads
```

Ejecuta el proyecto con:

```bash
cargo run
```

También puedes comprobar que el proyecto compile correctamente sin ejecutarlo:

```bash
cargo check
```

## Ejercicios

Las secciones incluyen ejemplos utilizados durante el curso y ejercicios para practicar los conceptos presentados.

Se recomienda:

1. Revisar el ejemplo de la sección.
2. Ejecutarlo con `cargo run`.
3. Modificar el código y experimentar con él.
4. Resolver el ejercicio correspondiente antes de continuar con la siguiente sección.

Los mensajes del compilador de Rust suelen incluir información y sugerencias útiles para identificar y corregir errores durante el aprendizaje.

## Recursos

* [Curso Profesional de Rust - Código Facilito](https://codigofacilito.com/cursos/rust-profesional/)
* [Sitio oficial de Rust](https://www.rust-lang.org/)
* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Documentación estándar de Rust](https://doc.rust-lang.org/std/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Cargo Book](https://doc.rust-lang.org/cargo/)

## Instructora

**Nina Maeva Mazadiego Cortés**

Software Engineer y desarrolladora de software con experiencia en Rust y otros lenguajes de programación.

GitHub: [@MaeMazcort](https://github.com/MaeMazcort)

## Código Facilito

Este repositorio contiene el material desarrollado para el [Curso Profesional de Rust de Código Facilito](https://codigofacilito.com/cursos/rust-profesional/).

El código está pensado como material de apoyo para acompañar las clases y facilitar la práctica de los conceptos vistos durante el curso.

: <<'DOC'
Sección 1.4 - Primer proyecto con Cargo

Descripción:
Este archivo muestra los comandos básicos para crear, revisar, compilar
y ejecutar un proyecto de Rust usando Cargo.

Instrucciones:
Ejecuta cada comando en orden. No ejecutes los comentarios.
La carpeta raíz del curso se llama CursoRust.
DOC

mkdir -p CursoRust/Bloque_1/Seccion_1_4
cd CursoRust

cargo new Bloque_1/Seccion_1_4/primer_proyecto_cargo

cat > Cargo.toml <<'TOML'
[workspace]
resolver = "3"
members = [
    "Bloque_1/Seccion_1_4/primer_proyecto_cargo",
]
TOML

cargo check -p primer_proyecto_cargo
cargo build -p primer_proyecto_cargo
cargo run -p primer_proyecto_cargo
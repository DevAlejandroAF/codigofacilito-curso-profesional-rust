/*
Sección 8.5 - Workspaces

Descripción:
Este crate de librería contiene lógica reutilizable para el workspace.
El binario workspace_cli dependerá de esta librería usando una dependencia local por path.

Diagrama conceptual:
CursoRust workspace
├── workspace_core -> librería con lógica reutilizable
└── workspace_cli  -> binario que usa workspace_core
*/

pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_lowercase()
}

pub fn crear_slug(nombre: &str) -> String {
    normalizar_nombre(nombre).replace(' ', "-")
}

/*
Ejercicio autónomo:
1. Agrega una función pública llamada contar_caracteres.
2. La función debe recibir &str y regresar usize.
3. Usa esa función desde workspace_cli.
4. Ejecuta cargo run -p workspace_cli desde CursoRust.
5. Observa que workspace_cli puede usar código de workspace_core.
6. Agrega un comentario explicando por qué conviene separar librería interna y binario.
*/
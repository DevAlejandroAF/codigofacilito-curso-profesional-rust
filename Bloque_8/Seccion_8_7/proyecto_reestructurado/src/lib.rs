/*
Sección 8.7 - Ejemplo práctico

Descripción:
Este archivo declara los módulos públicos de la librería.
La aplicación principal usará esta API para trabajar con productos y reportes.

Diagrama conceptual:
lib.rs
├── modelos.rs   -> define entidades del dominio
├── servicios.rs -> contiene lógica de negocio
└── reportes.rs  -> genera salidas legibles
*/

pub mod modelos;
pub mod servicios;
pub mod reportes;
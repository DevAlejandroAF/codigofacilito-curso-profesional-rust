: <<'DOC'
Sección 1.3 - Instalación y toolchain oficial

Descripción:
Este archivo contiene comandos de referencia para instalar y verificar
el entorno básico de Rust. No está pensado para ejecutarse completo sin revisar;
los comandos se deben ir ejecutando paso a paso según el sistema operativo.

Conceptos visuales:
toolchain = rustc + cargo + rustup + librería estándar + herramientas relacionadas

Flujo general:
1. Instalar rustup
2. Verificar rustc
3. Verificar cargo
4. Actualizar la toolchain estable
DOC

# macOS o Linux:
# curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# macOS, si aparece un error relacionado con linker o compilación nativa:
# xcode-select --install

# Windows:
# Descargar rustup-init.exe desde la página oficial de instalación de Rust.
# Durante la instalación, aceptar la instalación de las herramientas de Visual Studio si se solicitan.

rustup --version
rustc --version
cargo --version

rustup update
/*
Sección 9.5 - Introducción al diseño de herramientas CLI

Descripción:
Este archivo contiene la lógica principal de una herramienta CLI sencilla:
configuración, mensajes de ayuda y ejecución de comandos.

Criterios de diseño:
1. Mostrar ayuda clara.
2. Evitar panic! por argumentos faltantes.
3. Separar parsing de ejecución.
4. Usar mensajes de error accionables.

Ejercicio autónomo:
1. Agrega un comando despedir <nombre>.
2. Extiende el enum Accion con una variante Despedir.
3. Actualiza desde_args para parsear el nuevo comando.
4. Actualiza ayuda para documentarlo.
5. Actualiza ejecutar para imprimir la despedida.
6. Agrega un comentario explicando por qué main debe mantenerse pequeño.
*/

#[derive(Debug)]
pub enum Accion {
    Saludar { nombre: String },
    Ayuda,
}

#[derive(Debug)]
pub struct Config {
    accion: Accion,
}

impl Config {
    pub fn desde_args(args: &[String]) -> Result<Self, String> {
        if args.len() < 2 {
            return Ok(Self { accion: Accion::Ayuda });
        }

        match args[1].as_str() {
            "saludar" => {
                let nombre = args
                    .get(2)
                    .ok_or_else(|| String::from("Falta el nombre para saludar."))?;

                Ok(Self {
                    accion: Accion::Saludar {
                        nombre: nombre.clone(),
                    },
                })
            }
            "--help" | "-h" | "ayuda" => Ok(Self { accion: Accion::Ayuda }),
            otro => Err(format!("Comando desconocido: {otro}")),
        }
    }

    pub fn ayuda() -> &'static str {
        "Uso:\n  diseno_cli saludar <nombre>\n  diseno_cli --help"
    }
}

pub fn ejecutar(config: Config) {
    match config.accion {
        Accion::Saludar { nombre } => println!("Hola, {nombre}."),
        Accion::Ayuda => println!("{}", Config::ayuda()),
    }
}
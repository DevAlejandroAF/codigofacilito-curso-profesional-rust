/*
Sección 9.6 - Ejemplo práctico

Descripción:
Esta librería implementa una utilidad CLI pequeña para gestionar tareas.
Permite agregar, listar y marcar tareas como completadas usando un archivo de texto.

Diseño general:
1. Parsear argumentos en Config.
2. Representar comandos con un enum.
3. Leer y escribir tareas desde un archivo.
4. Propagar errores con Result.
5. Mantener main.rs pequeño.

Ejercicio autónomo:
1. Agrega un comando limpiar para eliminar las tareas completadas.
2. Agrega una variante Limpiar al enum Comando.
3. Actualiza Config::desde_args para reconocer limpiar.
4. Crea una función limpiar_completadas que reescriba el archivo sin tareas completadas.
5. Actualiza ejecutar para llamar esa función.
6. Agrega un comentario explicando cómo se separan parsing, lógica de negocio y persistencia.
*/

use std::error::Error;
use std::fs;
use std::path::Path;

const RUTA_TAREAS: &str = "tareas.txt";

#[derive(Debug)]
pub enum Comando {
    Agregar { descripcion: String },
    Listar,
    Completar { numero: usize },
}

#[derive(Debug)]
pub struct Config {
    comando: Comando,
}

#[derive(Debug, Clone)]
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Config {
    pub fn desde_args(args: &[String]) -> Result<Self, String> {
        let comando = args
            .get(1)
            .ok_or_else(|| String::from("Falta un comando: agregar, listar o completar."))?;

        match comando.as_str() {
            "agregar" => {
                let descripcion = args
                    .get(2)
                    .ok_or_else(|| String::from("Uso: utilidad_cli agregar <descripcion>"))?;

                Ok(Self {
                    comando: Comando::Agregar {
                        descripcion: descripcion.clone(),
                    },
                })
            }
            "listar" => Ok(Self {
                comando: Comando::Listar,
            }),
            "completar" => {
                let numero = args
                    .get(2)
                    .ok_or_else(|| String::from("Uso: utilidad_cli completar <numero>"))?
                    .parse::<usize>()
                    .map_err(|_| String::from("El número de tarea debe ser válido."))?;

                Ok(Self {
                    comando: Comando::Completar { numero },
                })
            }
            otro => Err(format!("Comando desconocido: {otro}")),
        }
    }
}

pub fn ejecutar(config: Config) -> Result<(), Box<dyn Error>> {
    match config.comando {
        Comando::Agregar { descripcion } => agregar_tarea(&descripcion)?,
        Comando::Listar => listar_tareas()?,
        Comando::Completar { numero } => completar_tarea(numero)?,
    }

    Ok(())
}

fn leer_tareas() -> Result<Vec<Tarea>, Box<dyn Error>> {
    if !Path::new(RUTA_TAREAS).exists() {
        return Ok(Vec::new());
    }

    let contenido = fs::read_to_string(RUTA_TAREAS)?;
    let tareas = contenido
        .lines()
        .filter_map(parsear_tarea)
        .collect();

    Ok(tareas)
}

fn guardar_tareas(tareas: &[Tarea]) -> Result<(), Box<dyn Error>> {
    let contenido = tareas
        .iter()
        .map(formatear_tarea)
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(RUTA_TAREAS, contenido)?;
    Ok(())
}

fn parsear_tarea(linea: &str) -> Option<Tarea> {
    let (estado, descripcion) = linea.split_once('|')?;

    Some(Tarea {
        completada: estado == "x",
        descripcion: descripcion.to_string(),
    })
}

fn formatear_tarea(tarea: &Tarea) -> String {
    let estado = if tarea.completada { "x" } else { " " };
    format!("{estado}|{}", tarea.descripcion)
}

fn agregar_tarea(descripcion: &str) -> Result<(), Box<dyn Error>> {
    let mut tareas = leer_tareas()?;

    tareas.push(Tarea {
        descripcion: descripcion.to_string(),
        completada: false,
    });

    guardar_tareas(&tareas)?;
    println!("Tarea agregada: {descripcion}");

    Ok(())
}

fn listar_tareas() -> Result<(), Box<dyn Error>> {
    let tareas = leer_tareas()?;

    if tareas.is_empty() {
        println!("No hay tareas registradas.");
        return Ok(());
    }

    for (indice, tarea) in tareas.iter().enumerate() {
        let marca = if tarea.completada { "[x]" } else { "[ ]" };
        println!("{} {} {}", indice + 1, marca, tarea.descripcion);
    }

    Ok(())
}

fn completar_tarea(numero: usize) -> Result<(), Box<dyn Error>> {
    let mut tareas = leer_tareas()?;
    let indice = numero
        .checked_sub(1)
        .ok_or("El número de tarea debe iniciar en 1.")?;

    let tarea = tareas
        .get_mut(indice)
        .ok_or("No existe una tarea con ese número.")?;

    tarea.completada = true;

    guardar_tareas(&tareas)?;
    println!("Tarea {numero} marcada como completada.");

    Ok(())
}
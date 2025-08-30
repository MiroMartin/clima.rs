use reqwest;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct Datos {
    current: Current,
}

#[derive(Debug, Deserialize)]
struct Current {
    time: String,
    interval: i32,
    apparent_temperature: f32,
    temperature_2m: f32,
}

fn main() -> Result<(), reqwest::Error> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=-36.3569&longitude=-56.7235&current=temperature_2m,apparent_temperature&forecast_days=1";

    let respuesta = reqwest::blocking::get(url)?;

    let cuerpo_json = respuesta.text()?;

    let datos: Datos = serde_json::from_str(&cuerpo_json).expect("error deserializando");

    println!(
        "La temperatura de San clemente es: {:?}",
        datos.current.temperature_2m
    );
    println!(
        "Y la sensacion termica es: {:?}",
        datos.current.apparent_temperature
    );
    Ok(())
}

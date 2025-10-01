use reqwest;
use serde::Deserialize;
use serde_json;

pub fn http_req(url: &str) -> Result<String, reqwest::Error> {
    let respuesta = reqwest::blocking::get(url)?;
    let cuerpo_json = respuesta.text()?;
    Ok(cuerpo_json)
}

#[derive(Debug, Deserialize)]
struct Datos {
    pub current: Current,
}

#[derive(Debug, Deserialize)]
struct Current {
    pub time: String,
    pub interval: i32,
    pub apparent_temperature: f32,
    pub temperature_2m: f32,
}

pub fn clima_json_request(cuerpo_json: &str) {

    let datos: Datos = serde_json::from_str(&cuerpo_json).expect("error deserializando");

    println!(
        "La temperatura de San Clemente es: {:?}",
        datos.current.temperature_2m
    );

    println!(
        "Y la sensacion termina es: {:?}",
        datos.current.apparent_temperature
    );

}

#[derive(Debug, Deserialize]
struct Result {
    zero: Locacion,
}

#[derive(Debug)]
struct Locacion {
    latitude: f64,
    longitude: f64,
}

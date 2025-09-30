use clima::*;

pub mod clima;

fn main() {

    //TODO: crear un modulo para las peticiones http, donde tambien se va a guardar esta url
    let url = "https://api.open-meteo.com/v1/forecast?latitude=-36.3569&longitude=-56.7235&current=temperature_2m,apparent_temperature&forecast_days=1";

    let respuesta: String = http_req(url).expect("No se pudo crear conexcion http");

    let _ = clima_json_request(&respuesta);
}

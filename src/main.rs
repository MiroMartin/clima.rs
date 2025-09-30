use clima::clima_json_request;
use std::process;

pub mod clima;

fn main() {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=-36.3569&longitude=-56.7235&current=temperature_2m,apparent_temperature&forecast_days=1";

    if let Err(e) = clima_json_request(url) {
        process::exit(1);
    }
}

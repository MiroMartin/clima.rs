use clima::clima_json_request;

pub mod clima;

fn main() {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=-36.3569&longitude=-56.7235&current=temperature_2m,apparent_temperature&forecast_days=1";

    let _ = clima_json_request(url);
}

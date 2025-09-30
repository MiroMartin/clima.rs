//json: result -> 0 -> latitude, longitude
//url: https://geocoding-api.open-meteo.com/v1/search?name=san+clemente+del+tuyu&count=10&language=en&format=json

//TODO: implementacion de la geolocalizacion, en primer momento con un struct (o los necesarios)
//para guardar el json de respuesta, y obtener las variables de latitud y longitud

#[derive(Debug, Deserialize]
struct Result {
    zero: Locacion,
}

#[derive(Debug)]
struct Locacion {
    latitude: f64,
    longitude: f64,
}


//TODO: implementacion de la funcionalidad de usar argumentos para que el usuario ingrese su
//localidad

//TODO: implementacion de la logica para que se comuniquen las funcionalidades, la geolocalizacion
//y el clima

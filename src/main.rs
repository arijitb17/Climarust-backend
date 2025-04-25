use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CityQuery {
    city: String,
    country: Option<String>,
}

#[derive(Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<GeoLocation>>,
}

#[derive(Deserialize, Clone)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
    name: String,
    country: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current_weather: WeatherData,
}

#[derive(Deserialize, Serialize)]
struct WeatherData {
    temperature: f64,
    windspeed: f64,
    weathercode: u8,
}

#[derive(Serialize)]
struct WeatherOutput {
    location: String,
    temperature: f64,
    windspeed: f64,
    weathercode: u8,
}

#[get("/weather")]
async fn get_weather(query: web::Query<CityQuery>) -> impl Responder {
    let city = &query.city;

    let geo_url = if let Some(country_code) = &query.country {
        format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&country={}",
            city, country_code
        )
    } else {
        format!("https://geocoding-api.open-meteo.com/v1/search?name={}", city)
    };

    if let Ok(resp) = reqwest::get(&geo_url).await {
        if let Ok(geo_data) = resp.json::<GeocodingResponse>().await {
            if let Some(locations) = geo_data.results {
                let selected_location = locations
                    .clone()
                    .into_iter()
                    .find(|loc| {
                        if let Some(ref country_code) = query.country {
                            loc.country.to_lowercase().contains(&country_code.to_lowercase())
                        } else {
                            true
                        }
                    })
                    .or_else(|| locations.into_iter().next());

                if let Some(location) = selected_location {
                    let weather_url = format!(
                        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                        location.latitude, location.longitude
                    );

                    if let Ok(weather_resp) = reqwest::get(&weather_url).await {
                        if let Ok(weather_data) = weather_resp.json::<WeatherResponse>().await {
                            let output = WeatherOutput {
                                location: format!("{}, {}", location.name, location.country),
                                temperature: weather_data.current_weather.temperature,
                                windspeed: weather_data.current_weather.windspeed,
                                weathercode: weather_data.current_weather.weathercode,
                            };

                            return HttpResponse::Ok().json(output);
                        }
                    }
                }
            }
        }
    }

    HttpResponse::InternalServerError().body("Could not get weather data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) 
            .service(get_weather)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

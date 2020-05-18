// use reqwest::Error;
use serde::{Deserialize, Serialize};
// use serde_json::Result;


const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/";
const WEATHER_API_KEY: &str = dotenv!("WEATHER_API_KEY");

// serde-rs / json for json to struct

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: f64,
    main: String,
    description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MainWeather {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64, // Maybe int
    humidity: f64  // Maybe int
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: i64,
    id: i64,
    country: String,
    sunrise: i64,
    sunset: i64
}


#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAPIResponse {
    coord: Coord,
    weather: Vec<Weather>,
    main: MainWeather,
    // visibility: f64, TODO change to optional 
    wind: Wind,
    clouds: Clouds,
    dt: f64,
    sys: Sys,
    timezone: f64,
    id: f64,
    cod: f64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    name: String,
    coord: Coord,
    country: String,
    timezone: f64,
    sunrise: f64,
    sunset: f64 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastResponse {
    cod: String,
    message: f64,
    cnt: i64,
    city: City,
    list: Vec<ForecastItem>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastItem {
    dt: i64,
    main: MainWeather,
    weather: Vec<Weather>,
    clouds: Clouds,
    wind: Wind,
    sys: ForecastSys, 
    dt_txt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastSys {
    pod: String 
} 


#[tokio::main]
pub async fn fetch_forecast_by_zip(zip: &str) -> Result<ForecastResponse, Box<dyn std::error::Error>> {
    let url = format!("{}forecast?zip={}&appid={}", BASE_URL, zip, WEATHER_API_KEY);
    println!("{}", url);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);
    let typed_resp: ForecastResponse = serde_json::from_str(&resp)?;
    Ok(typed_resp)
}

#[tokio::main]
pub async fn fetch_weather_by_zip(zip: &str) -> Result<WeatherAPIResponse, Box<dyn std::error::Error>> {
    let url = format!("{}weather?zip={}&appid={}", BASE_URL, zip, WEATHER_API_KEY);
    println!("{}", url);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    let typed_resp: WeatherAPIResponse = serde_json::from_str(&resp)?;
    Ok(typed_resp)
}


#[tokio::main]
pub async fn fetch_weather_by_city(city: &str) -> Result<WeatherAPIResponse, Box<dyn std::error::Error>> {
    let url = format!("{}weather?q={}&appid={}", BASE_URL, city, WEATHER_API_KEY);
    println!("{}", url);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    let typed_resp: WeatherAPIResponse = serde_json::from_str(&resp)?;
    Ok(typed_resp)
}
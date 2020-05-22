// use reqwest::Error;
use serde::{Deserialize, Serialize};
// use serde_json::Result;


const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/";
const WEATHER_API_KEY: &str = dotenv!("WEATHER_API_KEY");

// serde-rs / json for json to struct

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: f64,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: f64, // Maybe int
    pub humidity: f64  // Maybe int
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub r#type: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64
}


#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAPIResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub main: MainWeather,
    // visibility: f64, TODO change to optional 
    pub wind: Wind,
    pub clouds: Clouds,
    dt: f64,
    pub sys: Sys,
    timezone: f64,
    id: f64,
    cod: f64,
    pub name: String,
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
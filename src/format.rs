use crate::api::WeatherAPIResponse;
use chrono::{Local, TimeZone};

pub fn format_weather(weather: &WeatherAPIResponse) {
    let current_weather = &weather.weather.first();
    if let Some(weather) = current_weather {
        println!("{0: <6} | {1} ", "Curr", weather.main);
        println!("{0: <6} | {1} ", "Long", weather.description);
    }
    let main_weather = &weather.main;
    println!("{0: <6} | {1:.2} °F", "Temp", k_to_f(main_weather.temp));
    println!("{0: <6} | {1:.2} °F", "Feel", k_to_f(main_weather.feels_like));
    println!("{0: <6} | {1:.2} %", "Hum", main_weather.humidity);

    let wind = &weather.wind;
    println!("{0: <6} | {1:.2} MPH", "Wind", wind.speed);

    let sys= &weather.sys;
    println!("{0: <6} | {1}", "SRise", Local.timestamp(sys.sunrise, 0).format("%r"));
    println!("{0: <6} | {1}", "SSet", Local.timestamp(sys.sunset, 0).format("%r"));
}

pub fn k_to_f(kelvin: f64) -> f64 {
    kelvin * (9.0/5.0) - 459.67
}
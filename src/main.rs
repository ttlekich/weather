mod api;
use clap::{App, Arg};

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    let matches = App::new("weather")
        .version("0.0.1")
        .author("Travis Lekich <ttlekich@gmail.com")
        .about("Get your weather on the command line.")
        .arg(Arg::with_name("city").short("c").long("city").help("by city").takes_value(true).value_name("city"))
        .arg(Arg::with_name("zip").short("z").long("zip").help("by zip").takes_value(true).value_name("zip"))
        .arg(Arg::with_name("forecast").short("f").long("forecast").help("5 day forecast"))
        .arg(Arg::with_name("weather").short("w").long("weather").help("current weather"))
        .get_matches();

    if let Some(z) = matches.value_of("zip") {
        if matches.is_present("weather") {
            let resp = api::fetch_weather_by_zip(z);
            println!("{:#?}", resp);
        }
        if matches.is_present("forecast") {
            let resp = api::fetch_forecast_by_zip(z);
            println!("{:#?}", resp);
        }
    }

    if let Some(c) = matches.value_of("city") {
        if matches.is_present("weather") {
            let resp = api::fetch_weather_by_city(c);
            println!("{:#?}", resp);
        }
        if matches.is_present("forecast") {
            let resp = api::fetch_weather_by_city(c);
            println!("{:#?}", resp);
        }
    }
}

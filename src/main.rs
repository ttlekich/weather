mod api;
use clap::App;

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    let matches = App::new("weather")
        .version("0.0.1")
        .author("Travis Lekich <ttlekich@gmail.com")
        .about("Get your weather on the command line.")
        .arg("-z, --zip=[ZIP] 'by zip code.'")
        .arg("-c, --city=[CITY] 'by city.'")
        .arg("-f, --forecast 'forecast.'")
        .arg("-w, --weather 'current weather.'")
        .arg("-d... 'Turn debugging information on'")
        .get_matches();


    if let Some(z) = matches.value_of("zip") {
        // TODO how to get simple flags off of App?
        if let 1 = matches.occurrences_of("w") {
            let resp = api::fetch_weather_by_zip(z);
            println!("{:#?}", resp);
        }
        if let 1 = matches.occurrences_of("f") {
            let resp = api::fetch_forecast_by_zip(z);
            println!("{:#?}", resp);
        }
        let resp = api::fetch_weather_by_zip(z);
        println!("{:#?}", resp);
    }

    if let Some(c) = matches.value_of("city") {
        if let Some(_) = matches.value_of("weather") {
            let resp = api::fetch_weather_by_city(c);
            println!("{:#?}", resp);
        }
        if let Some(_) = matches.value_of("forecast") {
            let resp = api::fetch_weather_by_city(c);
            println!("{:#?}", resp);
        }
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("d") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }


    // Continued program logic goes here...
}
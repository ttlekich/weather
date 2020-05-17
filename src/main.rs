mod api;
use clap::App;

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    let matches = App::new("weather")
        .version("0.0.1")
        .author("Travis Lekich <ttlekich@gmail.com")
        .about("Get your weather on the command line.")
        .arg("-z, --zip=[ZIP] 'weather by zip code.'")
        .arg("-d... 'Turn debugging information on'")
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    if let Some(z) = matches.value_of("zip") {
        let resp = api::fetch_weather_by_zip(z);
        println!("{:#?}", resp);
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
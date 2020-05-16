const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
const WEATHER_API_KEY: &str = dotenv!("WEATHER_API_KEY");

// serde-rs / json for json to struct

#[tokio::main]
pub async fn fetch_weather_by_zip() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}?zip=77025&appid={}", BASE_URL, WEATHER_API_KEY);
    println!("{}", url);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);
    Ok(())
    // let resp = reqwest::get(&url)
    //     .await?;
        // .json::<HashMap<String, String>>()
        // .await?;
    // println!("RESPONSE: {:#?}", resp);
    // Ok(())
}

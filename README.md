# weather

A cli to show the weather.

## Usage

Sign up for a weather api key. Add that key to .env.

### Installation 

run `cargo build --release`

add `target/release/weather` to path or use at location.

### Current Weather

#### by zip

`wether -z <zip-code> -w`

_example_

`weather -z 77018 -w`

```
Curr   | Clear
Long   | clear sky
Temp   | 85.55 °F
Feel   | 93.63 °F
Hum    | 64.00 %
Wind   | 0.45 MPH
SRise  | 06:25:06 AM
SSet   | 08:12:05 PM
```

### 5-day forecast 

#### by zip 

`wether -z <zip-code> -f`

_example_

`weather -z 77018 -f`

```
Date         | Time   | Temp   | Weather
22-May-2020  | 10 pm  | 83.75  | Clouds
23-May-2020  |  1 am  | 80.60  | Clouds
23-May-2020  |  4 am  | 78.44  | Clouds
23-May-2020  |  7 am  | 79.25  | Clouds
23-May-2020  | 10 am  | 81.61  | Rain
23-May-2020  |  1 pm  | 87.66  | Clouds
23-May-2020  |  4 pm  | 90.46  | Clouds
23-May-2020  |  7 pm  | 85.05  | Clouds
23-May-2020  | 10 pm  | 80.56  | Clouds
24-May-2020  |  1 am  | 78.62  | Clouds
24-May-2020  |  4 am  | 77.43  | Rain
24-May-2020  |  7 am  | 77.43  | Clouds
24-May-2020  | 10 am  | 83.52  | Rain
24-May-2020  |  1 pm  | 87.73  | Rain
24-May-2020  |  4 pm  | 89.40  | Rain
24-May-2020  |  7 pm  | 83.89  | Clouds
24-May-2020  | 10 pm  | 79.52  | Rain
25-May-2020  |  1 am  | 78.67  | Rain
25-May-2020  |  4 am  | 77.59  | Rain
25-May-2020  |  7 am  | 71.11  | Rain
25-May-2020  | 10 am  | 73.98  | Rain
25-May-2020  |  1 pm  | 84.04  | Clouds
25-May-2020  |  4 pm  | 88.79  | Clouds
25-May-2020  |  7 pm  | 86.74  | Clouds
25-May-2020  | 10 pm  | 78.94  | Clear
26-May-2020  |  1 am  | 77.04  | Clouds
26-May-2020  |  4 am  | 76.41  | Rain
26-May-2020  |  7 am  | 75.69  | Rain
26-May-2020  | 10 am  | 75.67  | Rain
26-May-2020  |  1 pm  | 70.00  | Rain
26-May-2020  |  4 pm  | 74.44  | Rain
26-May-2020  |  7 pm  | 76.53  | Clouds
26-May-2020  | 10 pm  | 74.28  | Clouds
27-May-2020  |  1 am  | 73.06  | Clouds
27-May-2020  |  4 am  | 71.24  | Clouds
27-May-2020  |  7 am  | 71.69  | Clouds
27-May-2020  | 10 am  | 78.10  | Rain
27-May-2020  |  1 pm  | 84.04  | Clouds
27-May-2020  |  4 pm  | 85.60  | Rain
27-May-2020  |  7 pm  | 81.77  | Clouds
```

## Future

This was a quick project to reinforce topics learned with [Rustlings](https://github.com/rust-lang/rustlings). This is a bare-bones application, but PR's are welcome. Future changes might include: nicer formatting, more query options (city name, location, etc.), and more query filters (by date range, by time grain, etc.).
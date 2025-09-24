use crate::error::Error;
use crate::util::{AircraftData, AircraftMap, AircraftProvider, Bounds};

use attohttpc;
use serde_json::{self, Number, Value};
use std::collections::HashMap;
use std::convert::TryInto;

const ENDPOINT: &str = "https://api.adsb.lol/v2/point";

pub struct FlightRadar {
    base_url: String,
}

impl FlightRadar {
    pub fn new(radar_loc: &Bounds) -> Self {
        Self {
            base_url: format!(
                "{}/{}/{}/250",
                ENDPOINT, radar_loc.lat1, radar_loc.lon1
            ),
        }
    }
}

impl AircraftProvider for FlightRadar {
    fn get_aircraft(&mut self) -> Result<AircraftMap, Error> {
        let response = attohttpc::get(&self.base_url).send()?.error_for_status()?;

        let mut return_data = HashMap::new();
        let data: Value = serde_json::from_str(&response.text()?).unwrap();

        // Iterate through aircraft
        for value in data["ac"].as_array().unwrap() {

            let data: Value = serde_json::from_value(value.clone()).unwrap();

            let aircraft = AircraftData {
                squawk: data["squawk"].to_string().replace(" ", "").replace("\"", ""),
                callsign: data["flight"].to_string().replace(" ", "").replace("\"", ""),
                is_on_ground: data["alt_baro"] == "ground",
                latitude: data["lat"].as_f64().unwrap_or(0.0) as f32,
                longitude: data["lon"].as_f64().unwrap_or(0.0) as f32,
                heading: data["track"].as_f64().unwrap_or(0.0) as u32,
                ground_speed: data["gs"].as_f64().unwrap_or(0.0) as u32,
                timestamp: 0,
                altitude: if (data["alt_baro"] != "ground") {data["alt_baro"].as_f64().unwrap_or(0.0) as i32} else {0},
                model: data["t"].to_string().replace(" ", "").replace("\"", ""),
                hex: data["hex"].to_string().replace(" ", "").replace("\"", ""),
                origin: "ZZZZ".to_string(),
                destination: "ZZZZ".to_string(),
            };

            return_data.insert(data["hex"].to_string(), aircraft);
        }

        Ok(return_data)
    }

    fn get_name(&self) -> &str {
        "ADSB.lol"
    }
}

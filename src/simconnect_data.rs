use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::{
    clone::Clone,
    io::{Read, Write},
    thread::sleep,
    time::Duration,
    pin::Pin,
    boxed::Box,
    fs,
    writeln
};
use msfs::sim_connect::data_definition;

use crate::aircraft_movement;

#[data_definition]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AircraftData {
    #[name = "PLANE LATITUDE"]
    #[unit = "Degrees"]
    pub latitude: f64,
    #[name = "PLANE LONGITUDE"]
    #[unit = "Degrees"]
    pub longitude: f64,
    #[name = "PLANE ALTITUDE"]
    #[unit = "Feet"]
    pub altitude: f64,
    #[name = "PLANE BANK DEGREES"]
    #[unit = "Degrees"]
    pub bank: f64,
    #[name = "PLANE PITCH DEGREES"]
    #[unit = "Degrees"]
    pub pitch: f64,
    #[name = "PLANE HEADING DEGREES TRUE"]
    #[unit = "Degrees"]
    pub heading: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonData {
    pub data: Vec<AircraftData>,
    pub delta_times: Vec<f64>,
}

// TODO: Write a system to record (and play) data consinstantly, instead of every frame. Currently, the speed of the replay is determined by FPS. 
impl AircraftData {
    pub fn read_from_json(mut file: &fs::File, conn: &mut Pin<Box<msfs::sim_connect::SimConnect>>) {
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read file");
        let parsed: Value = serde_json::from_str(&data[..]).expect("Failed to parse JSON data");

        // This is only for testing purposes currently, and should be replaced.
        let parsed_length = parsed["data"].as_array().unwrap().len();
        for i in 0..parsed_length {
            aircraft_movement::update_aircraft(
                conn,
                AircraftData {
                    latitude: parsed["data"][i]["latitude"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    longitude: parsed["data"][i]["longitude"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    altitude: parsed["data"][i]["altitude"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    bank: parsed["data"][i]["bank"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    pitch: parsed["data"][i]["pitch"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    heading: parsed["data"][i]["heading"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                },
            );
            sleep(Duration::from_millis(parsed["delta_times"][i].as_f64().unwrap().round() as u64));
        }
    }
}

impl JsonData {
    pub fn write_to_json(&self, file: &mut fs::File) {
        let j = serde_json::to_string(&self).unwrap();
        writeln!(file, "{}", j).unwrap();
    }
}
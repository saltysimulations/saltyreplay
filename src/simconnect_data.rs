use crate::aircraft_movement;
use serde_json::{Result, Value};
use simconnect;
use std::{
    clone::Clone,
    io::{BufRead, BufReader, Read, Write},
    thread::sleep,
    time::Duration,
    vec,
};
use std::{fs, writeln};
#[derive(Debug, Copy, Clone)]
pub struct AircraftData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub bank: f64,
    pub pitch: f64,
    pub heading: f64,
}

// TODO: Rewrite the whole JSON writing system to use proper JSON syntax, instead of having an object each line and having it read that
// TODO: Write a system to record (and play) data consinstantly, instead of every frame. Currently, the speed of the replay is determined by FPS. 
impl AircraftData {
    pub fn initialize_data(conn: &mut simconnect::SimConnector) {
        conn.add_data_definition(
            0,
            "PLANE LATITUDE",
            "Degrees",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            u32::MAX,
        );
        conn.add_data_definition(
            0,
            "PLANE LONGITUDE",
            "Degrees",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            u32::MAX,
        );
        conn.add_data_definition(
            0,
            "PLANE ALTITUDE",
            "Feet",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            u32::MAX,
        );
        conn.add_data_definition(
            0,
            "PLANE BANK DEGREES",
            "Degrees",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            u32::MAX,
        );
        conn.add_data_definition(
            0,
            "PLANE PITCH DEGREES",
            "Degrees",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            u32::MAX,
        );
        conn.add_data_definition(
            0,
            "PLANE HEADING DEGREES TRUE",
            "Degrees",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            u32::MAX,
        );
        conn.request_data_on_sim_object(
            0,
            0,
            0,
            simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
            0,
            0,
            0,
            0,
        );
    }

    pub fn write_to_json(&self, file: &mut fs::File) {
        writeln!(
            file,
            "{{\"latitude\": {}, \"longitude\": {}, \"altitude\": {}, \"bankAngle\": {}, \"pitch\": {}, \"heading\": {}}}",
            self.latitude, self.longitude, self.altitude, self.bank, self.pitch, self.heading
        )
        .unwrap();
    }

    pub fn read_from_json(mut file: &fs::File, conn: &simconnect::SimConnector) {
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read file");
        let parsed: Value = serde_json::from_str(&data[..]).expect("Failed to parse JSON data");

        // This is only for testing purposes currently, and should be replaced.
        let mut index = 0;
        while index < 200 {
            aircraft_movement::update_aircraft(
                &conn,
                AircraftData {
                    latitude: parsed["data"][index]["latitude"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    longitude: parsed["data"][index]["longitude"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    altitude: parsed["data"][index]["altitude"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    bank: parsed["data"][index]["bankAngle"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    pitch: parsed["data"][index]["pitch"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                    heading: parsed["data"][index]["heading"]
                        .to_string()
                        .parse::<f64>()
                        .unwrap(),
                },
            );
            sleep(Duration::from_millis(36));
            index = index + 1;
        }
    }
}

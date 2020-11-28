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

    pub fn read_from_json(mut file: &fs::File, line_index: usize, conn: &simconnect::SimConnector) {
        /*let content = BufReader::new(file);
        let mut lines = content.lines();
        let line = lines.nth(line_index).expect("ttt").ok().unwrap();
        println!("{}", line);
        let parsed = json::parse(&line).unwrap();
        let data = AircraftData {
            latitude: parsed["latitude"].to_string().parse::<f64>().unwrap(),
            longitude: parsed["longitude"].to_string().parse::<f64>().unwrap(),
            altitude: parsed["altitude"].to_string().parse::<f64>().unwrap(),
            bank: parsed["bankAngle"].to_string().parse::<f64>().unwrap(),
            pitch: parsed["pitch"].to_string().parse::<f64>().unwrap(),
            heading: parsed["heading"].to_string().parse::<f64>().unwrap(),
        };
        data*/
        let mut data = String::new();
        file.read_to_string(&mut data).expect("sdfsdfsdf");
        let parsed: Value = serde_json::from_str(&data[..]).expect("sdf");
        println!("{}", parsed["data"][3]["latitude"]);
        let object_data = &parsed["data"][line_index];
        let mut data = AircraftData {
            latitude: object_data["latitude"].to_string().parse::<f64>().unwrap(),
            longitude: object_data["longitude"].to_string().parse::<f64>().unwrap(),
            altitude: object_data["altitude"].to_string().parse::<f64>().unwrap(),
            bank: object_data["bankAngle"].to_string().parse::<f64>().unwrap(),
            pitch: object_data["pitch"].to_string().parse::<f64>().unwrap(),
            heading: object_data["heading"].to_string().parse::<f64>().unwrap(),
        };
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

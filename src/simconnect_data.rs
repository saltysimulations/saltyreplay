use simconnect;
use std::io::Write;
use std::{fs, writeln};

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
            "{{latitude: {}, longitude: {}, altitude: {}, bankAngle: {}, pitch: {}, heading: {}}}",
            self.latitude, self.longitude, self.altitude, self.bank, self.pitch, self.heading
        )
        .unwrap();
    }

    // pub fn read_from_json(file: fs::File) {}
}

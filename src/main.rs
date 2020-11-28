mod aircraft_movement;
mod simconnect_data;

use simconnect;
use simconnect_data::AircraftData;
use std::{fs::File, fs::OpenOptions, mem::transmute_copy, thread::sleep, time::Duration};

fn main() {
    let mut conn = simconnect::SimConnector::new();
    conn.connect("FSReplay");

    let _file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("data.fsreplay")
        .unwrap();
    let mut file_to_read = File::open("data2.json").unwrap();

    AircraftData::initialize_data(&mut conn);
    AircraftData::read_from_json(&mut file_to_read, 1, &conn);

    loop {
        //aircraft_movement::update_aircraft(&conn, sim_data);

        match conn.get_next_message() {
            Ok(simconnect::DispatchResult::SimobjectData(data)) => unsafe {
                match data.dwDefineID {
                    0 => {
                        //let sim_data: AircraftData = transmute_copy(&data.dwData);
                        //sim_data.write_to_json(&mut file);
                    }
                    _ => (),
                }
            },
            _ => (),
        }
        sleep(Duration::from_millis(16));
    }
}

use simconnect;

use crate::simconnect_data::AircraftData;

pub fn update_aircraft(conn: &simconnect::SimConnector, mut data: AircraftData) {
    let pointer: *mut std::ffi::c_void = &mut data as *mut AircraftData as *mut std::ffi::c_void;
    conn.set_data_on_sim_object(
        0,
        0,
        0,
        0,
        std::mem::size_of::<AircraftData>() as u32,
        pointer,
    );
    println!("{:?}", data);
}

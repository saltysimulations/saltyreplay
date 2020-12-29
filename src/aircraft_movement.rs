use std::{pin::Pin, boxed::Box};
use msfs::sim_connect::SIMCONNECT_OBJECT_ID_USER;

use crate::simconnect_data::AircraftData;

pub fn update_aircraft(conn: &mut Pin<Box<msfs::sim_connect::SimConnect>>, data: AircraftData) {
    conn.set_data_on_sim_object(SIMCONNECT_OBJECT_ID_USER, &data).unwrap();
    println!("{:?}", data);
}

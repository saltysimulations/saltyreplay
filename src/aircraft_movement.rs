/*
 * SaltyReplay
 * Copyright (C) 2021 Salty Simulations
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::{pin::Pin, boxed::Box};
use msfs::sim_connect::SIMCONNECT_OBJECT_ID_USER;

use crate::simconnect_data::AircraftData;

pub fn update_aircraft(conn: &mut Pin<Box<msfs::sim_connect::SimConnect>>, data: AircraftData) {
    conn.set_data_on_sim_object(SIMCONNECT_OBJECT_ID_USER, &data).unwrap();
    println!("{:?}", data);
}

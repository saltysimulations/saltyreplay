#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use msfs::sim_connect::{Period, SimConnect, SimConnectRecv, SIMCONNECT_OBJECT_ID_USER};
use simconnect_data::{AircraftData, JsonData};
use std::{
  fs::{File, OpenOptions},
  sync::{Arc, Mutex},
  thread,
  thread::sleep,
  time::Duration,
  time::{SystemTime, UNIX_EPOCH},
};

mod aircraft_movement;
mod cmd;
mod replay_data;
mod simconnect_data;

fn main() {
  let mut _file_to_read = File::open("replays/replay_example.json").unwrap();

  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            PageChange { page } => {
              let page = &page[..];
              if let "replays" = page {
                tauri::event::emit(
                  &mut _webview.as_mut(),
                  String::from("replaydata"),
                  Some(replay_data::ReplayDataJson::get()),
                )
                .unwrap();
              }
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();

  let mut json_data = Arc::new(Mutex::new(JsonData {
    timestamp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs_f64()
      .round() as u64,
    data: Vec::new(),
    delta_times: Vec::new(),
  }));

  let last_time = std::cell::Cell::new(0.0);

  let mut sim = SimConnect::open("SaltyReplay", move |sim, recv| match recv {
    SimConnectRecv::SimObjectData(event) => match event.id() {
      0 => {
        // Calculate delta time
        let now = SystemTime::now()
          .duration_since(UNIX_EPOCH)
          .expect("Time went backwards")
          .as_secs_f64();
        if last_time.get() == 0.0 {
          last_time.set(now);
        }
        let delta_time = now - last_time.get();
        last_time.set(now);
        println!("{}", delta_time * 1000.0);

        // Write to file
        /* let data = event.into::<AircraftData>(sim).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open("test_replay.json")
            .unwrap();
        json_data.lock().unwrap().data.push(*data);
        json_data.lock().unwrap().delta_times.push(delta_time * 1000.0);
        json_data.lock().unwrap().write_to_json(&mut file); */
      }
      _ => {}
    },
    _ => {}
  })
  .unwrap();

  sim
    .request_data_on_sim_object::<AircraftData>(0, SIMCONNECT_OBJECT_ID_USER, Period::SimFrame)
    .unwrap();

  // Playing
  // AircraftData::read_from_json(&mut _file_to_read, &mut sim);

  loop {
    sim.call_dispatch().unwrap();
    sleep(Duration::from_millis(10));
  }
}

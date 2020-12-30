use msfs::sim_connect::{
    Period, SimConnect, SimConnectRecv, SIMCONNECT_OBJECT_ID_USER,
};
use actix_files as fs;
use actix_web::{App, HttpServer};
use simconnect_data::{AircraftData, JsonData};
use std::{
    fs::{File, OpenOptions},
    thread,
    thread::sleep,
    time::Duration,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};
use web_view::*;

mod aircraft_movement;
mod simconnect_data;

#[actix_web::main]
async fn set_up_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(fs::Files::new("/", "src/web/build").show_files_listing()))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}

fn main() {
    thread::spawn(|| match set_up_server() {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    });

    let mut _file_to_read = File::open("replay_example.json").unwrap();

    // TODO: Integrate the UI with the replay logic
    web_view::builder()
        .title("FSReplay")
        .content(Content::Url("http://127.0.0.1:5000"))
        .size(1200, 720)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            match arg {
                "rec_start" => {
                    println!("clicked rec start");
                }
                "rec_stop" => {
                    println!("clicked rec stop");
                }
                _ => unimplemented!(),
            }
            Ok(())
        })
        .run()
        .unwrap();

    let mut json_data = Arc::new(Mutex::new(JsonData {
        data: Vec::new(),
        delta_times: Vec::new(),
    }));

    let last_time = std::cell::Cell::new(0.0);

    let mut sim = SimConnect::open("SaltyReplay",  move |sim, recv| match recv {
        SimConnectRecv::SimObjectData(event) => match event.id() {
            0 => {
                // Calculate delta time
                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs_f64();
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
            },
            _ => {}
        },
        _ => {}
    }).unwrap();

    sim.request_data_on_sim_object::<AircraftData>(0, SIMCONNECT_OBJECT_ID_USER, Period::SimFrame).unwrap();

    // PLAYING (for testing purposes)
    AircraftData::read_from_json(&mut _file_to_read, &mut sim);

    // RECORDING (for testing purposes)
    loop {
        sim.call_dispatch().unwrap();
        sleep(Duration::from_millis(10));
    }
}
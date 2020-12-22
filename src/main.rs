mod aircraft_movement;
mod simconnect_data;

use actix_files as fs;
use actix_web::{App, HttpServer};
use simconnect;
use simconnect_data::AircraftData;
use std::{
    fs::{File, OpenOptions},
    thread,
    thread::sleep,
    time::Duration,
};
use web_view::*;

#[actix_web::main]
async fn set_up_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(fs::Files::new("/", "src/web/build").show_files_listing()))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}

fn main() {
    let mut conn = simconnect::SimConnector::new();
    conn.connect("FSReplay");

    thread::spawn(|| match set_up_server() {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    });

    let _file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("data.fsreplay")
        .unwrap();

    let mut _file_to_read = File::open("data2.json").unwrap();

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

    // AircraftData::initialize_data(&mut conn);

    // PLAYING (for testing purposes)
    // AircraftData::read_from_json(&mut _file_to_read, &conn);

    // RECORDING (for testing purposes)
    /* loop {
        //aircraft_movement::update_aircraft(&conn, sim_data);

        match conn.get_next_message() {
            Ok(simconnect::DispatchResult::SimobjectData(data)) => unsafe {
                match data.dwDefineID {
                    0 => {
                        let sim_data: AircraftData = transmute_copy(&data.dwData);
                        sim_data.write_to_json(&mut file);
                    }
                    _ => (),
                }
            },
            _ => (),
        }
        sleep(Duration::from_millis(16));
    } */
}

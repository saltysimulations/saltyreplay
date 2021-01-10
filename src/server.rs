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

extern crate glob;

use glob::glob;
use actix_files as fs;
use actix_web::{App, HttpServer, Responder, HttpResponse, get};
use std::{fs::File, io::Read};
use serde::{Deserialize, Serialize};

use crate::simconnect_data::JsonData;

#[actix_web::main]
pub async fn set_up_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_replays_json).service(fs::Files::new("/", "src/web/build").show_files_listing()))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}

#[get("/replaydata")]
async fn get_replays_json() -> impl Responder {
    let mut data = ReplayDataJson { data: Vec::new() };
    data.generate_new_json();
    HttpResponse::Ok().json(data)
}

#[derive(Serialize, Deserialize)]
pub struct ReplayData {
    timestamp: u64,
    length: f64,
    area: String,
    aircraft: String,
    id: u32,
}
#[derive(Serialize, Deserialize)]
pub struct ReplayDataJson {
    pub data: Vec<ReplayData>,
}

impl ReplayDataJson {
    pub fn generate_new_json(&mut self) {
        let files = ReplayDataJson::get_replay_files();
        for (i, mut file) in files.iter().enumerate() {
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let parsed: JsonData = serde_json::from_str(&data[..]).expect("Failed to parse JSON data");
            self.data.push(ReplayData {
                timestamp: parsed.get_timestamp(),
                length: parsed.get_length(),
                area: parsed.get_area(),
                aircraft: parsed.get_aircraft(),
                id: i as u32,
            });
        }
    }
    fn get_replay_files() -> Vec<File> {
        let mut files = Vec::new();
        for replay in glob("replays/*.json").expect("Failed to read glob pattern") {
            if let Ok(path) = replay {
                files.push(File::open(path).unwrap());
            }
        }
        files
    }
}
use serde::Deserialize;
use online::check;
use std::process::exit;
use reqwest::*;
use std::fs::File;

#[derive(Deserialize)]
pub struct Stalewall {
    pub source: String,
    pub url: String,
}

pub fn get_json() -> Stalewall {
    let response = blocking::get("https://stalewall.vercel.app/api").unwrap();
    let json: Stalewall = response.json().unwrap();
    json
}

pub fn get_image(path: String) {
   let api = blocking::get("https://stalewall.vercel.app/api").unwrap();
   let json: Stalewall = api.json().unwrap();
   let mut file = File::create(path).unwrap();
   let _ = blocking::get(json.url).unwrap().copy_to(&mut file);
}

pub fn check_network(timeout: u64) {
    let network_state: bool = check(Some(timeout)).is_ok();
    if network_state {
        println!("Connected to the internet!");
    } else {
        println!("Couldn't connect to the internet after $time");
        exit(1);
    }
}
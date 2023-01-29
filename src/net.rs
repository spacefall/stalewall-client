use serde::Deserialize;
use online::check;
use std::process;
use reqwest::blocking;

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

pub fn check_network(timeout: u64) {
    let network_state: bool = check(Some(timeout)).is_ok();
    if network_state {
        println!("Connected to the internet!");
    } else {
        println!("Couldn't connect to the internet after $time");
        process::exit(1);
    }
}
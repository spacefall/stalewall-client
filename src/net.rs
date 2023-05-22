//use online::check;
use ping::ping;
use reqwest::*;
use serde::Deserialize;
use std::fs::File;
use std::net::IpAddr;
use std::process::exit;
use std::time::Duration;

#[derive(Deserialize)]
pub struct Stalewall {
    pub source: String,
    pub url: String,
}

pub fn get_image(path: &String) {
    let api = blocking::get("https://stalewall.vercel.app/api").unwrap();
    let json: Stalewall = api.json().unwrap();
    let mut file = File::create(path).unwrap();
    blocking::get(json.url).unwrap().copy_to(&mut file).unwrap();
}

pub fn check_network(timeout: Duration) {
    // http://detectportal.firefox.com/success.txt
    let pingaddr: IpAddr = "34.107.221.82".parse().unwrap();
    let pinger = ping(pingaddr, Some(timeout), Some(112), None, None, None);

    // Basic error handling, if it doesn't error it just prints connected otherwise print not connected, the error and exit with code 1
    match pinger {
        Ok(_) => println!("Connected to the internet"),
        Err(e) => {
            println!("Not connected to the internet, error follows:\n{}", e);
            exit(1);
        }
    }
}

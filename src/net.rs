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

pub fn get_image(path: &String, queries: Option<String>) {
    // Temp if for queries, I'll put more options later so writing the queries manually isn't needed but for now (testing), it's good enough
    let url: String = if queries.is_some() {
        "https://stalewall.vercel.app/api".to_owned() + &queries.unwrap()
    } else {
        "https://stalewall.vercel.app/api".to_owned()
    };

    // Gets the data from "https://stalewall.vercel.app/api"
    let api = blocking::get(url);
    match api {
        Ok(res) => {
            let info: Result<Stalewall> = res.json();
            match info {
                Ok(j) => {
                    // Creates the image file
                    let mut file = File::create(path).unwrap();
                    // Downloads the image
                    let webpage = blocking::get(j.url); //.unwrap().copy_to(&mut file).unwrap();
                    match webpage {
                        Ok(mut image) => {
                            // this shouldn't happen and also i'm lazy so i'll do a neater thing later, but for now it's better than just an unwrap
                            image
                                .copy_to(&mut file)
                                .expect("There was an error while copying the image data");
                        }
                        Err(e) => {
                            println!(
                                "Couldn't download the image.\nCheck your connection and try again, if this issue persists, report it to the api repo's issues ({})\n\nError follows:\n{}",
                                "https://github.com/spacefall/stalewall-api/issues",
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "The response from the api wasn't a json.\nThis is probably a server bug and you should report it at {}\n\nError follows:\n{}",
                        "https://github.com/spacefall/stalewall-api/issues",
                        e
                    );
                }
            }
        }
        Err(e) => {
            println!("Something happened while getting a response from the api, check your connection and retry.\n\nError follows:\n{}", e);
            exit(1);
        }
    }
}

pub fn check_network(timeout: Duration) {
    // http://detectportal.firefox.com/success.txt
    let pingaddr: IpAddr = "34.107.221.82".parse().unwrap();
    let pinger = ping(pingaddr, Some(timeout), Some(112), None, None, None);

    // Basic error handling to check if the pc is connected to the internet
    match pinger {
        Ok(_) => println!("Connected to the internet"),
        Err(e) => {
            println!("Not connected to the internet\n\nError follows:\n{}", e);
            exit(1);
        }
    }
}

use anyhow::Result;
use std::fs::File;

use reqwest::blocking;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Stalewall {
    pub source: String,
    pub url: String,
}

pub fn dl_image(path: &str, url: String) -> Result<()> {
    let mut file = File::create(path)?;

    // Gets an image from the api; the response is a json, so it also gets deserialized using the Stalewall struct
    let api_json: Stalewall = blocking::get(url)?.json()?;

    // Downloads it and copy it into a file
    let mut image = blocking::get(api_json.url)?;
    image.copy_to(&mut file)?;

    Ok(())
}

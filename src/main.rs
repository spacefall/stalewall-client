#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use std::env;
use std::process::exit;

mod bg;
mod net;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the api (default: stalewall.vercel.app/api)
    #[arg(short, value_name = "URL")]
    url: Option<String>,
    
    /// String of queries to relay to the api
    #[arg(short, value_name = "QUERIES")]
    queries: Option<String>,

    /// Wallpaper position: 0 = Center, 1 = Tile, 2 = Stretch, 3 = Fit, 4 = Fill (default), 5 = Span
    #[arg(short, value_name = "POS")]
    pos: Option<u8>,

    /// Don't apply wallpaper to lockscreen
    #[arg(short)]
    lockscreen_skip: bool,

    /// Don't apply wallpaper to desktop
    #[arg(short)]
    desktop_skip: bool,
}

fn main() {
    let args = Args::parse();
    let api_url = args.url.unwrap_or("https://stalewall.vercel.app/api".to_string());
    let api_query = args.queries.unwrap_or(String::new());
    let full_url = format!("{api_url}{api_query}");
    
    println!("Final URL: {full_url}");
    
    if args.desktop_skip && args.lockscreen_skip {
        println!("Not applying wallpaper, exiting");
        exit(0);
    }

    // Network check
    online::check(Some(5)).expect("PC isn't connected to the internet");

    // Gets path to %temp%/stalewall_currentwall.jpg
    let tmppath = env::temp_dir().join("stalewall_currentwall.jpg");
    // And converts it to &str
    let wallpath = tmppath.to_str().unwrap();
    println!("Image path: {wallpath}");

    println!("Downloading image");
    net::dl_image(wallpath, full_url).expect("Couldn't download the image");

    println!("Applying wallpaper");
    bg::set(!args.desktop_skip, !args.lockscreen_skip, args.pos.unwrap_or(4), wallpath);
}

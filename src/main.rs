//use std::env;

//#[cfg(windows)]
//mod windows;

mod net;

use futures::executor::block_on;
use std::{env, time::Duration};
use wallpaper;
use windows::{
    core::*,
    Storage::{FileAccessMode, StorageFile},
};

fn main() {
    // Consts
    const TIMEOUT: Duration = Duration::new(5, 0);

    // check network
    println!(
        "Checking for connection to the internet, timeout: {}s",
        TIMEOUT.as_secs(),
    );
    net::check_network(TIMEOUT);

    // setup tempdir path
    let wallpath: String = env::temp_dir()
        .join("stalewall.jpg")
        .into_os_string()
        .into_string()
        .unwrap();
    // print path
    println!("Image path: {}", wallpath);
    // download image
    println!("Downloading image");
    net::get_image(&wallpath);
    // set desktop background
    println!("Applying desktop wallpaper");
    wallpaper::set_from_path(&wallpath).unwrap();
    // set lockscreen wallpaper
    block_on(set_lockscreen(wallpath)).unwrap();
}

async fn set_lockscreen(path: String) -> Result<()> {
    println!("Applying lockscreen wallpaper");
    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path))?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;
    windows::System::UserProfile::LockScreen::SetImageStreamAsync(&stream)?.await?;
    Ok(())
}

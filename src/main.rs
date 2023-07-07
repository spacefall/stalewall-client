mod net;

use futures::executor::block_on;
use std::{env, time::Duration};
use wallpaper;
use windows::{
    core::*,
    Storage::{FileAccessMode, StorageFile},
};

fn main() {
    // Network check timeout
    const TIMEOUT: Duration = Duration::new(5, 0);

    // Checks if pc is connected to the internet
    println!(
        "Checking for connection to the internet, timeout: {}s",
        TIMEOUT.as_secs(),
    );
    net::check_network(TIMEOUT);

    // Gets path to %temp%/stalewall_currentwall.jpg
    let wallpath: String = env::temp_dir()
        .join("stalewall_currentwall.jpg")
        .into_os_string()
        .into_string()
        .unwrap();
    println!("Image path: {}", wallpath);

    // Starts image download
    println!("Downloading image");
    net::get_image(&wallpath);
    println!("Image downloaded");

    // Applies wallpaper
    // To desktop
    println!("Applying desktop wallpaper");
    wallpaper::set_from_path(&wallpath).unwrap();
    // To Lockscreen
    println!("Applying lockscreen wallpaper");
    block_on(set_lockscreen(wallpath)).unwrap();
}

async fn set_lockscreen(path: String) -> Result<()> {
    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path))?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;
    windows::System::UserProfile::LockScreen::SetImageStreamAsync(&stream)?.await?;
    Ok(())
}

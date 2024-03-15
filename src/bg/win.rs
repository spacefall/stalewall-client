use windows::{
    core::{Result, HSTRING},
    Storage::{FileAccessMode, StorageFile},
    System::UserProfile::LockScreen,
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
        UI::Shell::{DesktopWallpaper, IDesktopWallpaper, DWPOS_CENTER,
                    DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN, DWPOS_STRETCH, DWPOS_TILE},
    },
};

// Set desktop wallpaper for Windows
pub(crate) unsafe fn set_desktop(path: &str, wallpos: u8) -> Result<()> {
    let wpos = match wallpos {
        0 => DWPOS_CENTER,
        1 => DWPOS_TILE,
        2 => DWPOS_STRETCH,
        3 => DWPOS_FIT,
        //4 => DWPOS_FILL,
        5 => DWPOS_SPAN,
        _ => DWPOS_FILL,
    };
    let hpath = &HSTRING::from(path);
    CoInitialize(None).unwrap();
    let wallpaper: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
    wallpaper.SetWallpaper(None, hpath)?;
    wallpaper.SetPosition(wpos)
}

// Set lockscreen wallpaper for Lockscreen
pub(crate) async fn set_lockscreen(path: &str) -> Result<()> {
    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path))?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;
    LockScreen::SetImageStreamAsync(&stream)?.await
}

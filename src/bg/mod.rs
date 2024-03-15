use futures::executor::block_on;

// Windows
#[cfg(windows)]
mod win;

#[cfg(windows)]
pub fn set(desktop: bool, lockscreen: bool, wallpos: u8, path: &str) {
    if desktop {
        unsafe { win::set_desktop(path, wallpos) }.expect("Desktop wallpaper could not be applied");
    }

    if lockscreen {
        block_on(win::set_lockscreen(path)).expect("Lockscreen wallpaper could not be applied");
    }
}

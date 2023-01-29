//use std::env;


//#[cfg(windows)]
//mod windows;

mod net;

use std::env::temp_dir;


fn main() {
    net::check_network(5);
    net::get_image(temp_dir().join("walle.jpg").into_os_string().into_string().unwrap());
    //let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    //let _ = futures::executor::block_on(apply_wallpaper_lockscreen(&args[1]));
}

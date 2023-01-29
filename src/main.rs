//use std::env;


//#[cfg(windows)]
//mod windows;

mod net;

fn main() {
    net::check_network(5);
    let json = net::get_json();
    println!("Url: {:?}\nSource: {:?}", json.url, json.source);
    //let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    //let _ = futures::executor::block_on(apply_wallpaper_lockscreen(&args[1]));
}

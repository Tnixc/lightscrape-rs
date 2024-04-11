mod utils;

use std::env;

// use crate::utils::{download_html, get_next_link, parse_content};
use crate::utils::{*};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a url!");
        return;
    } else if !&args[1].starts_with("https://") {
        println!("Please provide a valid url! (including https://)");
        return;
    }
    let url = &args[1];
    let body = download_html(&url);
    println!("{:?}", utils::get_title(&body));
    let n = get_read_now_link(&body, &url);
    println!("{:?}", n);
    let z = download_html(&n.to_string());
    println!("{:?}", get_next_link(&z, &url));
    // println!("{:?}", parse_content(&z));
}

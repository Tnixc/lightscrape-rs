mod utils;

use std::env;

// use crate::utils::{download_html, get_next_link, parse_content};
use crate::utils::*;

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
    let body_main = download_html(&url);
    println!("{:?}", utils::get_title(&body_main));
    let n = get_read_now_link(&body_main, &url);
    println!("{:?}", n);
    let chap1body = download_html(&n);
    fn recurse(url: &str, limit: u64) -> () {
        println!("{:?}", url);
        if limit == 0 {
            println!("limit reached");
            return;
        }
        let next = get_next_link(&download_html(&url.to_string()), &url.to_string());
        recurse(&next, limit - 1);
    }
    recurse(&get_next_link(&chap1body, &url), 10)
}

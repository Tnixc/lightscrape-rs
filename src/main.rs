mod utils;

use crate::utils::*;
use std::env;

extern crate html2md;

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
        let body = &download_html(&url.to_string());
        let next = get_next_link(body, &url.to_string());
        println!("{:?}", html2md::parse_html(parse_initial(body)));
        if limit == 0 {
            println!("limit reached");
            return;
        }
        recurse(&next, limit - 1);
    }
    recurse(&get_next_link(&chap1body, &url), 2)
}

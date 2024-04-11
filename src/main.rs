mod utils;

use crate::utils::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut limit = i32::MAX;
    if args.len() < 2 {
        println!("Please provide a url!");
        return;
    } else if !&args[1].starts_with("https://") {
        println!("Please provide a valid url! (including https://)");
        return;
    }

    let main_url = &args[1];
    if args.len() == 3 {
        limit = args[2].parse::<i32>().unwrap()
    }
    let main_body = download_html(&main_url);
    let title = get_title(&main_body);
    println!("Title: {:?}", title);

    let chapter_1_url = get_read_now_link(&main_body, &main_url);

    fn recurse(url: &str, limit: i32) -> () {
        println!("{:?}", url);
        let body = &download_html(&url.to_string());
        let next = get_next_link(body, &url.to_string());
        println!("{:?}", parse_content(body));
        if limit <= 0 {
            println!("limit reached");
            return;
        }
        recurse(&next, limit - 1);
    }

    recurse(&chapter_1_url, limit - 1)
}

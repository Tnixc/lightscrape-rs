mod utils;

use std::env;
use std::fs;
use std::path::Path;
use utils::*;

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

    if !Path::new("./res").exists() {
        let _ = fs::create_dir("./res");
    }

    let cover_url = get_cover_url(&main_body);

    let mut file = std::fs::File::create("cover.jpg").unwrap();
    reqwest::blocking::get(cover_url)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();

    let chapter_1_url = get_read_now_link(&main_body, &main_url);

    fn recurse(url: &str, limit: i32, i: i32) -> () {
        println!("{:?}", url);
        let body = &download_html(&url.to_string());
        let next = get_next_link(body, &url.to_string());

        let _ = fs::File::create("./res/".to_string() + i.to_string().as_str() + ".md");
        let _ = fs::write(
            "./res/".to_string() + i.to_string().as_str() + ".md",
            parse_content(body),
        );
        if limit <= 0 {
            println!("limit reached");
            return;
        }
        recurse(&next, limit - 1, i + 1);
    }

    recurse(&chapter_1_url, limit - 1, 1)
}

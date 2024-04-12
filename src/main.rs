mod utils;

use std::env;
use std::fs;
use std::path::Path;
use utils::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a url!");
        return;
    } else if !&args[1].starts_with("https://") {
        println!("Please provide a valid url! (including https://)");
        return;
    }

    let main_url = &args[1];
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

    let contents_urls = get_contents_link(&main_body, &main_url);

    println!("{:?}",get_contents_list(&contents_urls));

    println!("{:?}",get_list_links(&contents_urls)[0]);

    // let chapter_1_url = get_read_now_link(&main_body, &main_url);

    fn worker(url: &str, chapter: i32) -> () {
        println!("{:?}", url);
        let body = &download_html(&url.to_string());
        let next = get_next_link(body, &url.to_string());

        let _ = fs::File::create("./res/".to_string() + chapter.to_string().as_str() + ".md");
        let _ = fs::write(
            "./res/".to_string() + chapter.to_string().as_str() + ".md",
            parse_content(body),
        );
    }
}

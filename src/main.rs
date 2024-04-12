mod async_mode;
mod sync_mode;
mod utils;

use async_mode::*;
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

    let contents_url_1 = get_contents_link(&main_body, &main_url);

    let master: Vec<String> = get_contents_list(&contents_url_1);
    for i in master.iter() {
        println!("{:?}", i);
        let sublist = get_list_links(i);
        // println!("{:?}", thing)
        for item in sublist.iter() {
            worker(item);
        }
    }

    fn worker(chapter: &Chapter) -> () {
        let body = &download_html(&chapter.link);
        let path = "./res/".to_string() + "[" + &chapter.index + "] " + &chapter.title + ".md";
        let _ = fs::File::create(&path);
        let _ = fs::write(&path, parse_content(body));
        println!("{:?}", chapter.index);
    }
}

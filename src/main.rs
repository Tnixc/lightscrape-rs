mod async_mode;
mod sync_mode;
mod utils;

use async_mode::*;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use tokio::task;
use utils::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a url!");
        return;
    } else if !&args[1].starts_with("https://") {
        println!("Please provide a valid url! (including https://)");
        return;
    }

    let main_url = &args[1];
    let main_body = download_html(&main_url).await;
    let title = get_title(&main_body);
    println!("Title: {:?}", title);

    if !Path::new("./res").exists() {
        let _ = fs::create_dir("./res");
    }

    let cover_url = get_cover_url(&main_body);

    let mut image_file = std::fs::File::create("cover.jpg").unwrap();
    let image_data = reqwest::get(cover_url).await.unwrap().bytes();
    let _ = image_file.write_all(&image_data.await.unwrap());
    let contents_url_1 = get_contents_link(&main_body, &main_url);

    let master: Vec<String> = get_contents_list(&contents_url_1).await;

    let mut final_list: Vec<Chapter> = Vec::new();

    for page in master.iter() {
        final_list.append(&mut get_page_links(page).await);
    }

    for z in final_list.into_iter() {
        task::spawn(async {
            worker(z).await;
        });
    }
}
async fn worker(chapter: Chapter) -> () {
    println!("Started {:?}", chapter.index);
    let body = &download_html(&chapter.link).await;
    let path = "./res/".to_string() + "[" + &chapter.index + "] " + &chapter.title + ".md";
    let _ = fs::File::create(&path);
    let _ = fs::write(&path, parse_content(body));
    println!("Ended {:?}", chapter.index);
}

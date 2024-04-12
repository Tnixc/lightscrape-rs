mod async_mode;
mod sync_mode;
mod utils;

use async_mode::*;
use console::style;
use console::Term;
use dialoguer::{Input, Select};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use sync_mode::*;
use tokio::task;
use utils::*;

#[tokio::main]
async fn main() {
    let mode = vec![style("Async(Recommended)").green(), style("Sync").blue()];
    let selection = Select::new()
        .with_prompt(style("Choose a mode").bold().to_string())
        .items(&mode)
        .default(0)
        .interact()
        .unwrap();
    let main_url = Input::new()
        .with_prompt(
            style("Paste your link(has to be from the main page)")
                .bold()
                .to_string(),
        )
        .validate_with(|z: &String| {
            if z.starts_with("https://") {
                Ok(())
            } else {
                Err(style("Link doesn't start with https://").red().to_string())
            }
        })
        .interact()
        .unwrap();

    let term = Term::stdout();
    let _ = term.clear_screen();

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

    if selection == 1 {
        sync_main(&main_url, &main_body).await;
        return;
    }

    let contents_url_1 = get_contents_link(&main_body, &main_url);

    let master: Vec<String> = get_contents_list(&contents_url_1).await;
    println!("Total pages: {:?}, {:?}", master.len(), master);

    let mut final_list: Vec<Chapter> = Vec::new();
    for page in master.iter() {
        final_list.append(&mut get_page_links(page).await);
    }
    println!("Total chapters: {:?}, {:?}", final_list.len(), final_list);
    let mut handles = Vec::new();
    for z in final_list.into_iter() {
        handles.push(task::spawn(async {
            worker(z).await;
        }));
        sleep(std::time::Duration::from_millis(20));
    }
    futures::future::join_all(handles).await;
}

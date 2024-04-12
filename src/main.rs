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
        .with_prompt("Paste your link(has to be from the main page)")
        .validate_with(|z: &String| {
            if z.starts_with("https://") {
                Ok(())
            } else {
                Err("Link doesn't start with https://")
            }
        })
        .interact()
        .unwrap();

    let term = Term::stdout();
    let _ = term.clear_screen();

    if selection == 1 {
        let main_body = download_html(&main_url).await;

        let cover_url = get_cover_url(&main_body);

        let mut image_file = std::fs::File::create("cover.jpg").unwrap();
        let image_data = reqwest::get(cover_url).await.unwrap().bytes();
        let _ = image_file.write_all(&image_data.await.unwrap());

        sync_main(main_url).await;
        return;
    }

    let main_body = download_html(&main_url).await;
    let title = get_title(&main_body);
    println!("Title: {:?}", title);
    if !Path::new("./res").exists() {
        let _ = fs::create_dir("./res");
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
async fn worker(chapter: Chapter) -> () {
    println!("Started {:?} - {:?}", chapter.index, chapter.title);
    let body = &download_html(&chapter.link).await;
    let path;
    if chapter.index == "" {
        path = "./res/".to_string() + &chapter.title + ".md";
    } else {
        path = "./res/".to_string() + "[" + &chapter.index + "] " + &chapter.title + ".md";
    }
    let _ = tokio::fs::File::create(&path)
        .await
        .expect_err("msg err create");
    let _ = tokio::fs::write(&path, parse_content(body))
        .await
        .expect_err("msg err write");
    println!("Finished {:?} - {:?}", chapter.index, chapter.title);
    return;
}

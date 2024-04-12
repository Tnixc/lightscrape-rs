mod async_mode;
mod sync_mode;
mod utils;

use async_mode::*;
use console::style;
use console::Term;
use dialoguer::{Input, Select};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use sync_mode::*;
use tokio::sync::mpsc;
use tokio::task;
use utils::*;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let term = Term::stdout();
    let _ = term.clear_screen();

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
    let mut final_list: Vec<Chapter> = Vec::new();
    for page in master.iter() {
        final_list.append(&mut get_page_links(page).await);
    }

    let bar = ProgressBar::new(final_list.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/white}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>_"),
    );
    let start = Instant::now();

    let results = ProgressBar::new(final_list.len() as u64);
    results.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.green/white}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>_"),
    );

    let mut handles = Vec::new();
    let mut counta = 0;
    let (tx, mut rx) = mpsc::channel::<bool>(final_list.len());

    for z in final_list.into_iter() {
        let tx = tx.clone();
        handles.push(task::spawn(async {
            worker(z, tx).await;
        }));
        sleep(Duration::from_millis(10));
        counta += 1;
        bar.set_message(format!("Starting task for chapter {}", counta.to_string()));
        bar.inc(1);
    }
    bar.inc(1);
    for _ in 0..counta {
        let _ = rx.recv().await.unwrap();
        results.set_message(format!("Finished task for chapter {}", counta.to_string()));
        results.inc(1);
    }

    bar.finish_with_message(format!(
        "Started {} download tasks in {}",
        counta,
        HumanDuration(start.elapsed())
    ));

    futures::future::join_all(handles).await;
    results.finish_with_message(format!(
        "Finished {} download tasks in {}",
        counta,
        HumanDuration(start.elapsed())
    ))
}

mod async_mode;
mod sync_mode;
mod utils;

use async_mode::*;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{
    fs,
    fs::OpenOptions,
    io::Write,
    path::Path,
    thread,
    thread::sleep,
    time::{Duration, Instant},
};
use sync_mode::*;
use tokio::{sync::mpsc, task};
use utils::*;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let term = Term::stdout();
    let _ = term.clear_screen();

    let mut signals = Signals::new(&[SIGINT]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            println!(" Received interrupt signal {:?}", sig);
            std::process::exit(0);
        }
    });

    let modes = vec![style("Async(Recommended)"), style("Sync")];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(style("Choose a mode").bold().to_string())
        .items(&modes)
        .default(0)
        .interact()
        .unwrap();
    let main_url = Input::with_theme(&ColorfulTheme::default())
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

    let modes = vec![style("No"), style("Yes")];
    let keep_src = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(style("Keep source files? (.md)").bold().to_string())
        .items(&modes)
        .default(0)
        .interact()
        .unwrap();

    let main_body = download_html(&main_url).await;
    let title = get_title(&main_body);

    println!("{}", style(format!("\nTitle: {}", title)).blue().bold());

    if !Path::new("./res").exists() {
        let _ = fs::create_dir("./res");
    }
    if !Path::new("./res/src").exists() {
        let _ = fs::create_dir("./res/src");
    }

    let cover_url = get_cover_url(&main_body);

    let mut image_file = std::fs::File::create("./res/src/cover.jpg").unwrap();

    let image_data = reqwest::get(cover_url).await.unwrap().bytes();
    let _ = image_file.write_all(&image_data.await.unwrap());

    if selection == 1 {
        sync_main(&main_url, &main_body).await;

        let _ = generate_epub(&title, keep_src).await;

        println!(
            "{}",
            style(format!("\n Epub compiled at ./res/{}.epub", title))
                .green()
                .bold()
        );
        return;
    }

    let contents_url_1 = get_contents_link(&main_body, &main_url);

    let final_list: Vec<Chapter> = get_contents_list(&contents_url_1).await;

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
    let mut counta: u64 = 0;

    let (tx, mut rx) = mpsc::channel::<u64>(final_list.len());

    for z in final_list.into_iter() {
        let tx = tx.clone();

        handles.push(task::spawn(async move {
            worker(z, tx, &counta).await;
        }));

        sleep(Duration::from_millis(10));
        counta += 1;
        bar.set_message(format!("Starting task for chapter {}", counta.to_string()));
        bar.inc(1);
    }

    bar.set_position(counta);
    bar.finish_with_message(format!(
        "Started {} download tasks in {}",
        counta,
        HumanDuration(start.elapsed())
    ));

    let _ = tokio::fs::File::create("./res/src/SUMMARY.md").await;

    let mut summary_file = OpenOptions::new()
        .append(true)
        .open("./res/src/SUMMARY.md")
        .unwrap();

    for _ in 0..counta {
        let this = rx.recv().await.unwrap();
        results.set_message(format!("Finished task for chapter {}", this));
        results.inc(1);
    }
    for i in 0..counta {
        summary_file
            .write(format!("- [Chapter {}](./{}.md)\n", i + 1, i + 1).as_bytes())
            .unwrap();
    }

    futures::future::join_all(handles).await;

    results.set_position(counta);
    results.finish_with_message(format!(
        "Finished {} download tasks in {}",
        counta,
        HumanDuration(start.elapsed())
    ));

    let _ = generate_epub(&title, keep_src).await;

    println!(
        "{}",
        style(format!("\n Epub compiled at ./res/{}.epub", title))
            .green()
            .bold()
    )
}

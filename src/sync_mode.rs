use crate::utils::*;
use futures::future::*;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::{
    fs::OpenOptions,
    io::Write,
    time::{Duration, Instant},
};
use tokio::fs;

pub fn get_read_now_link(html: &String, url: &String) -> String {
    let line: String = html
        .split("\n")
        .into_iter()
        .filter(|&z| z.contains("readchapterbtn") || z.contains("Read Now"))
        .collect();

    let binding: String;
    if line.find("title") < line.find("class") {
        binding = get_substring_between(&line, "href=", "class")
            .unwrap_or_default()
            .replace("\"", "");
    } else {
        binding = get_substring_between(&line, "href=", "title")
            .unwrap_or_default()
            .replace("\"", "");
    }
    let res = binding;

    if res.starts_with("https://") {
        return res.trim().to_string();
    } else {
        let domain = url.split("/").collect::<Vec<&str>>()[2];
        return "https://".to_string() + domain + res.trim();
    }
}

pub fn get_next_link(html: &String, url: &String) -> String {
    let line: String = html
        .split("\n")
        .into_iter()
        .filter(|&z| z.contains("rel=\"next\""))
        .collect();

    let binding: String;
    if line.find("title") < line.find("class") {
        binding = get_substring_between(&line, "href=", "class")
            .unwrap_or_default()
            .replace("\"", "")
            .replace("><i", "");
    } else {
        binding = get_substring_between(&line, "href=", "title")
            .unwrap_or_default()
            .replace("\"", "");
    }
    let res = binding;

    if res.starts_with("https://") {
        return res.trim().to_string();
    } else if !res.contains("javascript") {
        let domain = url.split("/").collect::<Vec<&str>>()[2];
        return "https://".to_string() + domain + res.trim();
    } else {
        return "".to_string();
    }
}

pub async fn sync_main(main_url: &String, main_body: &String) -> () {
    let chapter_1_url = get_read_now_link(&main_body, &main_url);

    let started = Instant::now();
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]",
                "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
                "[-==-]",
            ]),
    );

    let _ = fs::File::create("./res/src/SUMMARY.md").await;
    let mut summary_file = OpenOptions::new()
        .append(true)
        .open("./res/src/SUMMARY.md")
        .unwrap();

    let i = recurse(chapter_1_url, 1, spinner).await;
    for z in 0..i {
        let _ = summary_file.write(format!("- [Chapter {}](./{}.md)\n", z + 1, z + 1).as_bytes());
    }

    let duration = started.elapsed();
    let human_readable = HumanDuration(duration);
    println!("Took {}", human_readable);
}

fn recurse(url: String, i: i32, spinner: ProgressBar) -> BoxFuture<'static, i32> {
    async move {
        let body = &download_html(&url.to_string()).await;
        let title = get_title(body);
        let next = get_next_link(body, &url.to_string());
        spinner.set_message(format!("Downloading chapter {}", i));
        if next.is_empty() {
            spinner.finish_with_message(format!("Finished downloading {} chapters", i));
            return i;
        }
        let path = "./res/src/".to_string() + i.to_string().as_str() + ".md";
        let content = parse_content(body);
        let _ = fs::File::create(&path).await;
        let _ = fs::write(
            &path,
            "# ".to_string() + title.as_str() + "\n \n" + content.as_str(),
        )
        .await;

        return recurse(next, i + 1, spinner).await;
    }
    .boxed()
}

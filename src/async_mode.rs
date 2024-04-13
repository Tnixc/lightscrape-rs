use crate::utils::*;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::time::{Duration, Instant};
use tokio::{fs, fs::File, sync::mpsc, time::sleep};

#[derive(Debug)]
pub struct Chapter {
    pub title: String,
    pub link: String,
    pub index: String,
}

pub async fn worker(chapter: Chapter, tx: mpsc::Sender<u64>, counta: &u64) -> () {
    let body = &download_html(&chapter.link).await;
    let path = "./res/src/".to_string() + (counta + 1).to_string().as_str() + ".md";
    let _ = File::create(&path).await;
    let mut content = parse_content(body);
    loop {
        if content.contains("All of our servers are busy right now") {
            sleep(Duration::from_millis(100)).await;
            content = parse_content(&download_html(&chapter.link).await);
        } else {
            break;
        }
    }

    let _ = fs::write(
        &path,
        "# ".to_string() + chapter.title.as_str() + "\n \n" + content.as_str(),
    )
    .await;
    tx.send(counta.clone()).await.unwrap();
    return;
}

pub async fn get_page_links(url: &String, body: &String) -> Vec<Chapter> {
    let reduced = get_substring_between(&body, "<ul class=\"chapter-list\">", "</ul>").unwrap();
    let mut n: Vec<Chapter> = reduced
        .split("</li>")
        .into_iter()
        .map(|z| {
            let mut link = get_substring_between(&z, "href=", "title")
                .unwrap()
                .replace("\"", "")
                .trim()
                .to_owned();
            if link.starts_with("https://") {
                link = link.trim().to_string();
            } else {
                let domain = url.split("/").collect::<Vec<&str>>()[2];
                link = "https://".to_string() + domain + link.trim();
            }

            let title = get_substring_between(&z, "title=", ">")
                .unwrap()
                .replace("\"", "")
                .trim()
                .replace("&#x2019;", "'")
                .to_owned();
            let index;
            if z.contains("data-orderno=") {
                index = get_substring_between(&z, "data-orderno=", ">")
                    .unwrap()
                    .replace("\"", "")
                    .trim()
                    .to_owned();
            } else if z.contains("Chapter") {
                index = get_substring_between(&z, "Chapter", ":")
                    .unwrap_or_default()
                    .trim()
                    .to_owned();
            } else {
                index = "".to_string();
            }

            return Chapter { title, link, index };
        })
        .collect::<Vec<Chapter>>();

    n.pop();
    return n;
}

pub fn get_contents_link(html: &str, url: &str) -> String {
    let line: String = html
        .split("\n")
        .into_iter()
        .filter(|&z| z.contains("chapter-latest-container"))
        .collect();

    let binding = get_substring_between(&line, "href=", ">").unwrap_or_default();
    let res = binding.replace("\"", "");

    if res.starts_with("https://") {
        return res.trim().to_string();
    } else {
        let domain = url.split("/").collect::<Vec<&str>>()[2];
        return "https://".to_string() + domain + res.trim();
    }
}

pub async fn get_contents_list(url: &String) -> Vec<Chapter> {
    let mut index = 2;
    let mut vec_of_body = Vec::new();

    let started = Instant::now();
    let spinner = ProgressBar::new_spinner();

    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {spinner:.yellow} {msg}")
            .unwrap()
            .tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]",
                "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
                "[-==-]",
            ]),
    );

    spinner.set_message(format!("Downloading page {}", (index - 1).to_string()));

    // Page 1
    vec_of_body.push(download_html(url).await);

    loop {
        let next_url = url.clone() + "?page=" + index.to_string().as_str();
        let res = download_html(&next_url).await;
        if res.contains("Page Not Found")
            || get_substring_between(&res, "class=\"chapter-list", "</ul>")
                .unwrap_or_default()
                .len()
                < 10
        {
            break;
        }

        vec_of_body.push(res);

        spinner.set_message(format!("Downloading page {}", index.to_string()));
        index += 1;
    }

    let mut final_list: Vec<Chapter> = Vec::new();

    for page_body in vec_of_body.iter() {
        final_list.append(&mut get_page_links(url, page_body).await);
    }

    spinner.finish_with_message(format!(
        "Downloaded {} content pages in {}",
        index - 1,
        HumanDuration(started.elapsed())
    ));

    return final_list;
}

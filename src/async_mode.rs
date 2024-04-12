use crate::utils::*;

#[derive(Debug)]
pub struct Chapter {
    pub title: String,
    pub link: String,
    pub index: String,
}

pub async fn worker(chapter: Chapter) -> () {
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

pub async fn get_page_links(url: &String) -> Vec<Chapter> {
    let res = download_html(&url).await;
    let reduced = get_substring_between(&res, "<ul class=\"chapter-list\">", "</ul>").unwrap();
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

pub async fn get_contents_list(url: &String) -> Vec<String> {
    let mut index = 2;
    let mut vec = Vec::new();
    vec.push(url.clone());
    println!("1");
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
        println!("{:?}", index);
        vec.push(next_url);
        index += 1;
    }
    return vec;
}

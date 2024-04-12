use crate::utils::*;

use futures::future::*;

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

    fn recurse(url: String, i: i32) -> BoxFuture<'static, i32> {
        async move {
            println!("{:?}", url);
            let body = &download_html(&url.to_string()).await;
            let next = get_next_link(body, &url.to_string());
            if next.is_empty() {
                return i;
            }
            let path = "./res/".to_string() + "[" + i.to_string().as_str() + "] " + ".md";
            let _ = tokio::fs::File::create(&path).await;
            let _ = tokio::fs::write(&path, parse_content(body)).await;
            return recurse(next, i + 1).await;
        }
        .boxed()
    }
    recurse(chapter_1_url, 1).await;
}

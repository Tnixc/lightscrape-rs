use crate::utils::*;

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
    } else {
        let domain = url.split("/").collect::<Vec<&str>>()[2];
        return "https://".to_string() + domain + res.trim();
    }
}

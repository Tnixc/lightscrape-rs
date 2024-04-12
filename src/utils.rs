use core::panic;

use regex::Regex;
extern crate reqwest;

pub fn download_html(url: &String) -> String {
    let req = reqwest::blocking::get(url);
    let res = match req {
        Ok(body) => body,
        Err(er) => panic!("Problem with downloading html: {:?}", er),
    };
    let body = res.text();
    return match body {
        Ok(z) => z,
        Err(er) => panic!("Problem with converting body to text {:?}", er),
    };
}

pub fn get_title(html: &String) -> &str {
    let in_tag = get_substring_between(&html, "<title>", "</title>").unwrap_or_default();
    if in_tag.contains("|") {
        let end = in_tag.find("|").unwrap_or_default();
        return &in_tag[..end].trim();
    } else {
        return &in_tag.trim();
    }
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

pub fn get_contents_list(url: &String) -> Vec<String> {
    let mut index = 2;
    let mut vec = Vec::new();
    vec.push(url.clone());

    loop {
        let next_url = url.clone() + "?page=" + index.to_string().as_str();
        let res = download_html(&next_url);
        if res.contains("Page Not Found") {
            break;
        }
        println!("{:?}", index);
        vec.push(next_url);
        index += 1;
    }
    return vec;
}

pub fn get_list_links(url: &String) -> Vec<[String; 2]> {
    let res = download_html(&url);
    let reduced = get_substring_between(&res, "<ul class=\"chapter-list\">", "</ul>").unwrap();
    let n: Vec<[String; 2]> = reduced
        .split("<span")
        .into_iter()
        .map(|z| {
            let link = get_substring_between(&z, "href=", "title")
                .unwrap()
                .replace("\"", "")
                .trim()
                .to_owned();
            let title = get_substring_between(&z, "title=", ">")
                .unwrap()
                .replace("\"", "")
                .trim()
                .to_owned();
            return [link, title];
        })
        .collect::<Vec<[String; 2]>>();

    return n;
}

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

pub fn parse_content(html: &str) -> String {
    fn parse_initial(html: &str) -> String {
        let z = get_substring_between(&html, "itemprop=\"description\"", "chapternav").unwrap();
        let pattern = Regex::new(r"<script>.*?</script>").unwrap();
        let res = pattern.replace_all(z, "");
        return res.to_string();
    }

    let binding = html2md::parse_html(&parse_initial(html));
    return binding
        .trim()
        .trim_start_matches("\\>")
        .trim()
        .replace("*This chapter upload first at **Novel Fire***", "")
        .to_string();
}

pub fn get_substring_between<'a>(text: &'a str, start: &'a str, end: &'a str) -> Option<&'a str> {
    if !text.contains(start) || !text.contains(end) {
        return Some(text);
    }
    let first = text.find(start)?;
    let last = text[first..].find(end)?;
    return Some(&text[first + start.len()..last + first]);
}

pub fn get_cover_url(html: &str) -> String {
    let line = get_substring_between(html, "<figure", "</figure>").unwrap_or_default();
    let url = get_substring_between(line, "data-src=", "alt").unwrap_or_default();
    return url.replace("\"", "").trim().to_string();
}

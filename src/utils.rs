use core::panic;
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

pub fn get_read_now_link(html: &String, url: &String) -> String {
    let line: String = html
        .split("\n")
        .into_iter()
        .filter(|&z| z.contains("readchapterbtn") || z.contains("Read Now"))
        .collect();

    let binding = get_substring_between(&line, "href=", "class")
        .unwrap_or_default()
        .replace("\"", "");
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
    let res = get_substring_between(&line, "href=\"", "\"")
        .unwrap_or_default()
        .replace("\"", "");

    if res.starts_with("https://") {
        return res.to_string();
    } else {
        return "https://".to_string() + url.split("/").collect::<Vec<&str>>()[2] + res.as_str();
    }
}

// pub fn parse_content(html: &str) -> &str {
//    let z = get_substring_between(&html, "itemprop=\"description\"", "chapternav");
//    return z.unwrap();
// }

// fn get_substring_between<'a>(str: &'a str, start: &'a str, end: &'a str) -> &'a str {
//     if !str.contains(start) || !str.contains(end) {
//         panic!("Could not get substring between {} and {}", start, end);
//     }
//     return str.split(start).collect::<Vec<&str>>()[1]
//         .split(end)
//         .collect::<Vec<&str>>()[0];
// }

pub fn get_substring_between<'a>(text: &'a str, start: &'a str, end: &'a str) -> Option<&'a str> {
    if !text.contains(start) || !text.contains(end) {
        return None;
    }
    let first = text.find(start)?;
    let last = text[first..].find(end)?;
    return Some(&text[first + start.len()..last + first]);
}

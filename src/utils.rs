use core::panic;

extern crate reqwest;

pub fn download_html(url: &String) -> String {
    let req = reqwest::blocking::get(url);
    let res = match req {
        Ok(body) => body,
        Err(er) => panic!("Problem with downloading html: {:?}", er)
    };
    let body = res.text();
    return match body {
        Ok(z) => z,
        Err(er) => panic!("Problem with converting body to text {:?}", er)
    }
}

pub fn get_title(html: &String) -> &str {
    let start = "<title>";
    let end = "</title>";
    let start_position = html.find(start);

    if start_position.is_some() {
        let start_position = start_position.unwrap() + start.len();
        let html = &html[start_position..];
        let end_position = html.find(end).unwrap_or_default();
        let in_tag = &html[..end_position];
        if in_tag.contains("|") {
            let end = in_tag.find("|").unwrap_or_default();
            return &in_tag[..end]
        } else {
            return in_tag;
        }
    } else {
        panic!("something went wrong with parsing the html!")
    }
}

pub fn get_read_now_link(html: &String) -> String {
    let lines = html.split("\n");
    let filtered_lines : String = lines.into_iter().filter(|&z| z.contains("readchapterbtn") || z.contains("Read Now")).collect();
    return filtered_lines;
}

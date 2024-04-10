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

pub fn get_title(html: &String) -> String {
    let in_tag = get_substring_between(&html, "<title>", "</title>");   
    if in_tag.contains("|") {
        let end = in_tag.find("|").unwrap_or_default();
        return in_tag[..end].to_string();
    } else {
        return in_tag;
    }
}

pub fn get_read_now_link(html: &String) -> String {
    let lines = html.split("\n");
    return lines.into_iter().filter(|&z| z.contains("readchapterbtn") || z.contains("Read Now")).collect();
    // return filtered_lines;
}

fn get_substring_between(str: &String, start: &str, end: &str) -> String {
let start_bytes = str.find(start).unwrap_or(0); 
let end_bytes = str.find(end).unwrap_or(str.len()); 
let result = &str[start_bytes..end_bytes]; 
    return result.to_string();
}

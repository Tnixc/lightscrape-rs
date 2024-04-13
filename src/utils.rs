use core::panic;
use mdbook::renderer::RenderContext;
use mdbook::MDBook;
use mdbook_epub;
use mdbook_epub::errors::Error;
use std::path::PathBuf;

use regex::Regex;

pub async fn download_html(url: &String) -> String {
    let req = reqwest::get(url).await;
    let res = match req {
        Ok(body) => body,
        Err(er) => panic!("Problem with downloading html: {:?}", er),
    };
    let body = res.text().await;
    return match body {
        Ok(z) => z,
        Err(er) => panic!("Problem with converting body to text {:?}", er),
    };
}

pub fn get_title(html: &String) -> String {
    let in_tag = get_substring_between(&html, "<title>", "</title>")
        .unwrap_or_default()
        .replace("&#x27;", "'")
        .replace(" - Top Novel Updates", "")
        .replace(" - Web Novel Pub", "");
    if in_tag.contains("|") {
        let end = in_tag.find("|").unwrap_or_default();
        return in_tag[..end].trim().to_owned();
    } else {
        return in_tag.trim().to_owned();
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

pub fn generate_epub(title: String) -> Result<(), Error> {
    let book_dir = PathBuf::from("./res/");

    let book = MDBook::load(&book_dir).unwrap();
    let mut config = book.config.clone();
    config.book.title = Some(title);
    config.book.src = PathBuf::from("./res/src");

    let ctx = RenderContext::new(
        book.root.clone(),
        book.book.clone(),
        config,
        book_dir.clone(),
    );

    let _ = mdbook_epub::generate(&ctx)?;
    println!("EPUB generated");
    Ok(())
}

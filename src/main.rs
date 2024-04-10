mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a url!");
        return;
    } else if !&args[1].starts_with("https://") {
        println!("Please provide a valid url! (including https://)");
        return;
    }
    let url = &args[1];
    let body = utils::download_html(&url);
    println!("{:?}", utils::get_title(&body));
    println!("{:?}", utils::get_read_now_link(&body, &url));
}

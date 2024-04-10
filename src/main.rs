mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a url!");
        return;
    }
    let word = &args[1];
    println!("{}", utils::download_html(word))
}

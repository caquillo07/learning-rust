use std::env;
use std::process;
use ch12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);

    if let Err(err) = ch12_minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

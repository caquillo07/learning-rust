use std::env;
use std::process;
use ch13_3_improving_minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);

    if let Err(err) = ch13_3_improving_minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

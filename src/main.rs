use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("got an error while processing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("In file {}", config.file_path);

    minigrep::run(config).unwrap_or_else(|err| {
        println!("got an error {}", err);
        process::exit(1);
    })
}

use std::env;
use std::process;
use minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        println!("Probelem parsing args {}", err);
        process::exit(1);
    });

    println!("Query for = {}", config.query);
    println!("Filename = {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}

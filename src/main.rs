use std::process;
use std::env;
use minigrep::Config;

#[allow(unused)]
fn main() {
    let args: Vec<String> = get_args();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn get_args() -> Vec<String> {
    env::args().collect()
}
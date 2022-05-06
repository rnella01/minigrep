use std::process;
use std::env;
use minigrep::Config;

#[allow(unused)]
fn main() {
    let args: Vec<String> = get_args();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments {}", err);
        process::exit(1);
    });
    minigrep::run(config);
}

fn get_args() -> Vec<String> {
    env::args().collect()
}
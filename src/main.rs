use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = get_args();
    // println!("{:?}", args);
    let pattern = &args[1];
    let filename = &args[2];
    println!("Pattern: {}, Filename: {}", pattern, filename);
    let file_content = fs::read_to_string(filename)
                                .expect("unable to read file");
    println!("file content:\n{}", file_content);
}

fn get_args() -> Vec<String> {
    env::args().collect()
}

fn parse_config(args: &[String]) -> (&str, &str) {
    (&args[1], &args[2])
}

// fn get_file_content(filename: &str) -> Result<String, std::io::Error>{
//     fs::read_to_string(filename)
// }

#[cfg(test)]
mod tests {
    use super::*;

}
use std::fs;

use std::error::Error;

pub struct Config {
    pattern: String,
    filename: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough parameters");
        }
        // cloned each of the arg values since
        // the `args` variable in main is the owner of the argument values
        // and is only letting the `parse_config` function borrow them,
        // which means we’d violate Rust’s borrowing rules
        // if `Config` tried to take ownership of the values in args
        Ok(Config {
            pattern: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(config.filename)?;
    println!("pattern: {}", config.pattern);
    println!("file content:\n{}", file_content);
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_resullt() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick all 3!
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}
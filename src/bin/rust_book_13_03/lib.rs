use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, mate!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query , file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.file_path)?;
    
    println!("\nWith the following text: \n {content}");

    Ok(())
}

// example for not use Vec, faster and the same understanding
// pub fn search<'a>(query: &str, contents: &'a str) {/* -> Vec<&'a str> { */

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
            println!("Eureka! It worked! :D\n");
            println!("fn search()\n");
        }
    }

    result
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {

    let query = query.to_lowercase();
    
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
            println!("fn search_case_insensitive()\n");
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}


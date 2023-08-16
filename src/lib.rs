use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough argument to call");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        return err.to_string();
    });

    let result = if config.ignore_case {
        ingore_case_search(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for res in result {
        println!("{}", res);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents));
    }
}

pub fn ingore_case_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut grep = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            grep.push(line);
        }
    }
    grep
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut grep = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            grep.push(line);
        }
    }
    grep
}

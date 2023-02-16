use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String>{
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }

        let query: String;
        let file_path: String;

        let keywords: Vec<&String>;
        let flags: Vec<char>;

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        
        (keywords, flags) = parse_args(args);

        for flag in flags {
            match flag {
                'i' => ignore_case = true,
                flag => return Err(format!("invalid argument flag: {flag}")),
            }
        }

        query = keywords[1].clone();
        file_path = keywords[2].clone();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn parse_args(args: &[String]) -> (Vec<&String>, Vec<char>) {
    let mut keywords = Vec::new();
    let mut flags = Vec::new();

    for arg in args {
        if arg[0..1].eq_ignore_ascii_case("-") { 
            for flag in arg.chars() {
                if flag.eq_ignore_ascii_case(&'-') {
                    continue;
                }
                flags.push(flag);
            }
        } else {
            keywords.push(arg);
        }
    }

    (keywords, flags)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
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

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn ignore_case_argument() {

        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn get_keywords_and_opts_from_args() {
        let args = &["-i".to_string(), "to".to_string(), "sample.txt".to_string()];

        let query1 = String::from("to");
        let query2 = String::from("sample.txt");
        let query = vec![&query1, &query2];

        let contents = vec!['i'];

        assert_eq!(parse_args(args), (query, contents));
    }
}

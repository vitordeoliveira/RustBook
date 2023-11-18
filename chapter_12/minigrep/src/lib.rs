use std::{env, error::Error, fs};

// We’ll cover trait objects in Chapter 17.
// For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // REFACTOR
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    // ? will return the error value from the current function for the caller to handle.
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut res = vec![];
    // // let mut res = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }
    // res
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut res = Vec::new();
    // let query = query.to_lowercase();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         res.push(line)
    //     }
    // }
    // res
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // We’re also going to change the function name from new to build because many programmers expect new functions to never fail. When Config::build is communicating to main
    // Returning a Result Instead of Calling panic!
    //
    //
    // Args can be any type that implements the Iterator type and returns String items.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // We don’t care about the value of the environment variable, just whether it’s set or unset, so we’re checking is_ok rather than using unwrap, expect
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
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

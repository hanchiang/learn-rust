use std::fs;
use std::error::Error;
use std::env;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>
    {
        // First value is the name of the program
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        // If the environment variable CASE_INSENSITIVE is not set, make the search case sensitive
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// Box<dyn Error> is a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text: \n{}\n", contents);

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

// We tell Rust that the data returned by the search function will live as long as
// the data passed into the search function in the contents argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Didn't get a query string")]
    fn config_new_without_query_should_fail() {
        let args = vec!(String::from("program name"));
        Config::new(args.into_iter()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Didn't get a file name")]
    fn config_new_without_filename_should_fail() {
        let args = vec!(String::from("program name"), String::from("text"));
        Config::new(args.into_iter()).unwrap();
    }

    #[test]
    fn config_new_with_3_args_should_pass() {
        let args = vec!(String::from("program name"), String::from("text"), String::from("filename"));
        let result = Config::new(args.into_iter()).unwrap();
        assert_eq!(result, Config { query: String::from("text"), filename: String::from("filename"), case_sensitive: true });
    }

    #[test]
    #[should_panic(expected = "NotFound")]
    fn run_with_invalid_filename_should_fail() {
        let args = vec!(String::from("program name"), String::from("text"), String::from("filename"));
        let result = Config::new(args.into_iter()).unwrap();
        run(result).unwrap();
    }

    #[test]
    fn run_with_valid_filename_should_pass() {
        let args = vec!(String::from("program name"), String::from("text"), String::from("poem.txt"));
        let result = Config::new(args.into_iter()).unwrap();
        let response = run(result).unwrap();
        assert_eq!(response, ());
    }

    #[test]
    fn search_should_return_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}
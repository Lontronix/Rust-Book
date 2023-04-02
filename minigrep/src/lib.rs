use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    query: String,
    file_path: String,
	ignore_case: bool
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();

		let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config { query, file_path, ignore_case }
    }
}

pub enum ParseConfigError {
    TooFewArgsError
}

pub fn parse_config(args: &[String]) -> Result<Config, ParseConfigError> {

    if args.len() != 3 {
        return Err(ParseConfigError::TooFewArgsError);
    }

    Ok(Config::new(args))
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

	let results = if config.ignore_case {
		search_ignoring_case(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};

	for line in results {
    	println!("{line}");
	}

    Ok(())
}

pub fn search_ignoring_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query.to_lowercase()) {
			results.push(line);
		}
	}

	results
}

// we need to specify 'a as a lifetime here because the returned vector
// will have string slices from contents and never from query
//
// aka the data returned by this function must live as long as contents does
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

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
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents))
	}

	#[test]
	fn case_insensitive() {
		let query = "DUCT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search_ignoring_case(query, contents))
	}

}

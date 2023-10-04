use std::fs;
use std::error::Error;
use std::env;

pub struct Args {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Args {query, file_path, ignore_case})
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(args.file_path)?;

    let results = if args.ignore_case {
        search_case_insensitive(&args.query, &content)
    } else {
        search(&args.query, &content)
    };

    
    for (n, line) in results {
        println!("{}: {}", n+1, line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    for (n, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((n, line));
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (n, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((n, line));
        }
    }

    results
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
Pick three.
Duct tape.";

        assert_eq!(vec![(1, "safe, fast, productive.")], search(query, contents));
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
            vec![(0, "Rust:"), (3, "Trust me.")],
            search_case_insensitive(query, contents)
        );
    }
}
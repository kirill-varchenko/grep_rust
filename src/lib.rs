use std::fs;
use std::error::Error;
use std::env;

pub struct Args {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Args {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Args {query, file_path, ignore_case})
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(args.file_path)?;

    let results = search(&args.query, &content, !args.ignore_case);

    for (n, line) in results {
        println!("{}: {}", n+1, line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str, case: bool) -> Vec<(usize, &'a str)> {
    let query_lowercase = query.to_lowercase();
    let predicate: Box<dyn FnMut(&(usize, &str)) -> bool> = match case {
        true => Box::new(|(_, line): &(usize, &str)| line.contains(query)),
        false => Box::new(|(_, line): &(usize, &str)| line.to_lowercase().contains(&query_lowercase))
    };

    content
        .lines()
        .enumerate()
        .filter(predicate)
        .collect()
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

        assert_eq!(vec![(1, "safe, fast, productive.")], search(query, contents, true));
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
            search(query, contents, false)
        );
    }
}
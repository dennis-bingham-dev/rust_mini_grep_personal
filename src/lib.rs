use std::{fs, error::Error };
use clap::Parser;

pub fn parse_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse(); 

    Ok(args)
}

pub fn run(terms: Terms) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(terms.file)?;

    let results = if terms.case_sensitive {
        search_case_sensitive(&terms.query, &contents)
    } else {
        search_case_insensitive(&terms.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Desired search term
    #[clap(short, long, value_parser)]
    pub query: String,

    /// Target file
    #[clap(short, long, value_parser, default_value = "")]
    pub file: String,

    /// Case sensitive ("f" for false & "t" for true)
    #[clap(short, long, value_parser, default_value = "f")]
    pub case_sensitive: String,
}

pub struct Terms {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Terms {
    /**
     * This method takes in args and returns a terms object to use within our program for
     * readability.
     */
    pub fn new(args: &Args) -> Result<Terms, &str> {
        if args.query == "" {
            return Err("missing query argument...")
        }

        if args.file == "" {
            return Err("missing file name...")
        }
        
        if args.case_sensitive.to_lowercase() != "f" && args.case_sensitive.to_lowercase() != "t" {
            return Err("invalid case sensitive argument...")
        };

        let query = args.query.clone();
        let file = args.file.clone();
        let case_sensitive = parse_true_or_false_argument(&args.case_sensitive.clone());
        Ok(Terms { query, file, case_sensitive })
    }
}

pub fn parse_true_or_false_argument(case_sensitive: &str) -> bool {
    let is_case_sensitive = if case_sensitive.to_lowercase() == "f" {
        false
    } else {
        true 
    };

    is_case_sensitive
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
       if line.contains(&query) {
          results.push(line);
       } 
    }

    results
}

pub fn search_case_insensitive<'contents>(query: &str, contents: &'contents str) -> Vec<&'contents str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
       if line.to_lowercase().contains(&query) {
          results.push(line);
       } 
    }

    results
}


// TODO: Figure out how to move tests into their own file....
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "Rust:\nsafe, fast, productive\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
            );
    }
}
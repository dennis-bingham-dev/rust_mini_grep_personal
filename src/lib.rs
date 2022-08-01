use std::{fs, error::Error };
use clap::Parser;

pub fn parse_args() -> Result<Args, Box<dyn Error>> {
    Ok(Args::parse())
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
        if let Err(e) = validate_args(args) {
            return Err(e);
        }

        
        Ok(create_terms_struct(args))
    }
}

pub fn create_terms_struct(args: &Args) -> Terms {
    let (query, file, case_sensitive) = extract_terms_properties_from_args(args);
    
    Terms { query, file, case_sensitive }
}


pub fn extract_terms_properties_from_args(args: &Args) -> (String, String, bool) {
    (args.query.clone(), args.file.clone(), parse_true_or_false_argument(&args.case_sensitive.clone()))
}

/**
 * Validates command-line arguments before use.
 */
pub fn validate_args(args: &Args) -> Result<bool, &str> {
    if args.query == "" {
        return Err("missing query argument...")
    }

    if args.file == "" {
        return Err("missing file name...")
    }
    
    if validate_case_sensitive_character(args.case_sensitive.clone()) {
        return Err("invalid case sensitive argument...")
    };
    
    Ok(true)
}

pub fn validate_case_sensitive_character(case_sensitive: String) -> bool {
    return case_sensitive.to_lowercase() != "f" && case_sensitive.to_lowercase() != "t";
}

pub fn parse_true_or_false_argument(case_sensitive: &str) -> bool {
    let is_case_sensitive = if case_sensitive.to_lowercase() == "f" {
        false
    } else {
        true 
    };

    is_case_sensitive
}

pub fn search_case_sensitive<'contents>(query: &str, contents: &'contents str) -> Vec<&'contents str> {
    let mut results = Vec::<&str>::new();
    
    for line in contents.lines() {
       if line.contains(&query) {
          results.push(line);
       } 
    }

    results
}

pub fn search_case_insensitive<'contents>(query: &str, contents: &'contents str) -> Vec<&'contents str> {
    let query = query.to_lowercase();
    let mut results = Vec::<&str>::new();
    
    for line in contents.lines() {
       if line.to_lowercase().contains(&query) {
          results.push(line);
       } 
    }

    results
}

// pub fn progress_bar(progress: f64, total: f64) {
//    let progress_char = '>';
//    let percent = 100 * (progress / total);
//    Ok(())
//}


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

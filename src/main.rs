use std::process;

use minigrep::{ Terms, Args };


fn main() {
    let args: &Args = &minigrep::parse_args().unwrap();
    
    let terms: Terms = Terms::new(args).unwrap_or_else(|err: &str| {
        // eprintln! will print to the error stream.
        eprintln!("Problem with provided search terms: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(terms) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
                              Rust:
                              Safe, fast, productive.
                              Pick three.";

        assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
    }
}

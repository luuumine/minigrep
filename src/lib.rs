//! `minigrep` - a tiny `grep`-like tool for searching lines in text files.
//!
//! This crate is implements the `minigrep` crate as described by *[The Rust Programming
//! Language](https://doc.rust-lang.org/book/)* book (chapters 12-13).
//!
//! The core search logic is included as reusable functions.
//!
//! # Features
//!
//! - Simple case-sensitive search via [`search`]
//! - Optional case-insensitive search via [`search_case_insensitive`]
//!
//! # Command-line usage
//!
//! The binary expects two positional arguments: a query string and a file path.
//! Case sensitivity is controlled via the `IGNORE_CASE` environment variable.
//!
//! ```bash
//! IGNORE_CASE=1 minigrep to poem.txt
//! ```
//! By default, search is case-insensitive.

/// Search for lines in `contents` that contain `query` (case-sensitive).
///
/// This function iterates over all lines of the input and returns those that contain the given
/// query string as a substring. Matching is done via `srt::contains`.
///
/// # Arguments
///
/// - `query`: the string to look for
/// - `contents`: the text to search in, usually the contents of a file
///
/// # Returns
///
/// A `Vec<&str>` containing references to the matching lines insides `contents`. The returned
/// slices borrow from `contents`, so `contents` must outlive the result.
///
/// # Examples
///
/// ```rust
/// use minigrep::search;
///
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// let matches = search(query, contents);
/// assert_eq!(matches, vec!["safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search for lines in `contents` that contain `query` (case-insensitive).
///
/// This is the case-insensitive counterpart to [`search`]. Both the query and each line are
/// lowercased before comparison.
///
/// This function iterates over all lines of the input and returns those that contain the given
/// query string as a substring. Matching is done via `srt::contains`.
///
/// # Arguments
///
/// - `query`: the string to look for
/// - `contents`: the text to search in, usually the contents of a file
///
/// # Returns
///
/// A `Vec<&str>` containing references to the matching lines insides `contents`. The returned
/// slices borrow from `contents`, so `contents` must outlive the result.The matching lines are
/// returned in their original form (not lowercased).
///
/// # Examples
///
/// ```rust
/// use minigrep::search_case_insensitive;
///
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
///
/// let matches = search_case_insensitive(query, contents);
/// assert_eq!(matches, vec!["Rust:","Trust me."]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();

    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_one() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive_many() {
        let query = "body";
        let contents = "\
nobody
frog
somebody
body
adult
";

        assert_eq!(vec!["nobody", "somebody", "body"], search(query, contents));
    }

    #[test]
    fn case_insensitive_one() {
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

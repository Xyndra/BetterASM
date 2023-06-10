use lazy_static::lazy_static;
use regex::{Captures, Regex};

///
/// This function takes a _**one-line**_ string and returns it with all whitespace in front of
/// and behind it removed.
///
/// # Example:
///
/// ```rust
/// let str_with_whitespace: String = "     hello world    ".to_string();
/// let str_without_whitespace: String = util::strip_whitespace(str_with_whitespace);
///
/// assert_eq!(str_without_whitespace, "hello world");
/// ```
///
pub fn strip_whitespace (str_with_whitespace: String) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^\s*(\S*(( = ((['"].*['"])|(\S*)))|(: ((['"].*['"])|(\S*))))?)\s*(;.*)?$"#).unwrap();
    }
    if !RE.is_match(&str_with_whitespace) {
        panic!("Invalid line: {}", str_with_whitespace);
    }
    let captures: Captures = RE.captures(&str_with_whitespace).unwrap();
    let instruction: String = captures.get(1).map_or_else(|| "".to_string(), |m| m.as_str().to_string());
    let comment: String = captures.get(11).map_or_else(|| "".to_string(), |m| m.as_str().to_string());
    if !instruction.is_empty() {
        if !comment.is_empty() {
            return vec![instruction, comment];
        }
        return vec![instruction, "".to_string()];
    } else if !comment.is_empty() {
        return vec!["".to_string(), comment];
    }
    return vec![];
}

//! `hello-world` — a tiny command-line greeting tool.
//!
//! Usage:
//! ```text
//! hello-world            -> "Hello, world!"
//! hello-world <name>     -> "Hello, <name>!"
//! ```
//!
//! If more than one argument is given, only the first one is used.

/// Builds the greeting for the given name.
///
/// Returns the default greeting when `name` is `None`, otherwise greets the
/// supplied name. The logic lives in its own function (rather than inline in
/// `main`) so it can be unit-tested without spawning a process.
///
/// # Arguments
///
/// * `name` — the name to greet, or `None` to fall back to "world".
fn greeting(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Hello, {name}!"),
        None => "Hello, world!".to_string(),
    }
}

/// Entry point: reads the first CLI argument (if any) and prints the greeting.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{}", greeting(args.get(1).map(String::as_str)));
}

#[cfg(test)]
mod tests {
    use super::greeting;

    /// No argument means the default "world" greeting.
    #[test]
    fn greets_world_when_no_name_is_given() {
        assert_eq!(greeting(None), "Hello, world!");
    }

    /// A single name is greeted as-is.
    #[test]
    fn greets_the_given_name() {
        assert_eq!(greeting(Some("Devajyoti")), "Hello, Devajyoti!");
    }

    /// Names with spaces are preserved in the greeting.
    #[test]
    fn greets_multi_word_names() {
        assert_eq!(
            greeting(Some("Devajyoti Sarkar")),
            "Hello, Devajyoti Sarkar!"
        );
    }
}

//! `hello-world` — a tiny command-line greeting tool.
//!
//! Usage:
//! ```text
//! hello-world                    -> "Hello, world!"
//! hello-world <name>             -> "Hello, <name>!"
//! hello-world <word> <word> ...  -> "Hello, <word> <word> ...!"
//! hello-world ""                 -> "Hello, there!"
//! ```
//!
//! Behaviour contract:
//! * All arguments are joined with a single space, so unquoted multi-word
//!   names work the same as quoted ones: `hello-world Rust is great` and
//!   `hello-world "Rust is great"` produce identical output.
//! * Empty-string arguments are ignored; if the remaining name is empty
//!   (i.e. only empty arguments were given), the greeting is "Hello, there!".
//! * If *any* argument is not valid UTF-8 (only possible on Unix), nothing
//!   usable is left to greet, so the program prints "Hello, there!" (the
//!   user *did* try to give a name) and exits successfully.
//! * The name is embedded **verbatim** — no escaping, normalization, or
//!   truncation.

/// Builds the greeting for the given name.
///
/// * `None` — no arguments were given at all → default "Hello, world!"
/// * `Some("")` — a name was given but is empty → "Hello, there!"
/// * `Some(name)` — greets the supplied name verbatim.
///
/// The logic lives in its own function (rather than inline in `main`) so it
/// can be unit-tested without spawning a process.
fn greeting(name: Option<&str>) -> String {
    match name {
        Some("") => "Hello, there!".to_string(),
        Some(name) => format!("Hello, {name}!"),
        None => "Hello, world!".to_string(),
    }
}

/// Resolves the raw CLI arguments into the name to greet.
///
/// * No arguments → `None` (nothing was given at all → "Hello, world!").
/// * Any argument that is not valid UTF-8 → `Some("")` (something *was*
///   given, but nothing usable → "Hello, there!"). A partially broken name
///   is never half-greeted: one failure fails the whole batch.
/// * Otherwise → all non-empty arguments joined with a single space (which
///   is `Some("")` when only empty arguments were given).
fn name_from_args(args: &[std::ffi::OsString]) -> Option<String> {
    if args.is_empty() {
        return None;
    }
    // Lossless UTF-8 conversion: if any argument fails, fail the whole batch
    // and fall back to the empty name ("Hello, there!"). `into_string`
    // consumes its input, so clone each argument first.
    let args: Vec<String> = match args.iter().map(|a| a.clone().into_string()).collect() {
        Ok(args) => args,
        Err(_) => return Some(String::new()),
    };
    let non_empty: Vec<&str> = args
        .iter()
        .filter(|a| !a.is_empty())
        .map(String::as_str)
        .collect();
    Some(non_empty.join(" "))
}

/// Entry point: resolves the CLI arguments into a name and prints the greeting.
fn main() {
    let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    let name = name_from_args(&args);
    println!("{}", greeting(name.as_deref()));
}

#[cfg(test)]
mod tests {
    use super::{greeting, name_from_args};
    use std::ffi::OsString;

    // ---------------------------------------------------------------------
    // greeting()
    // ---------------------------------------------------------------------

    /// No usable name means the default "world" greeting.
    #[test]
    fn greets_world_when_no_name_is_given() {
        assert_eq!(greeting(None), "Hello, world!");
    }

    /// A name was given but is empty → "Hello, there!".
    #[test]
    fn greets_there_when_the_name_is_empty() {
        assert_eq!(greeting(Some("")), "Hello, there!");
    }

    /// A single name is greeted as-is.
    #[test]
    fn greets_the_given_name() {
        assert_eq!(greeting(Some("Alambayana")), "Hello, Alambayana!");
    }

    /// Names with spaces are preserved in the greeting.
    #[test]
    fn greets_multi_word_names() {
        assert_eq!(greeting(Some("Rust is great")), "Hello, Rust is great!");
    }

    /// The name is embedded verbatim for a wide range of inputs: accented
    /// Latin, CJK, right-to-left scripts, emoji (including ZWJ sequences and
    /// skin-tone modifiers), combining marks, zero-width and non-breaking
    /// spaces, newlines, format-string metacharacters, and degenerate names
    /// (whitespace-only, very long).
    ///
    /// The byte-length check proves there is no truncation: the output must
    /// be exactly `Hello, ` + the name's bytes + `!`.
    #[test]
    fn name_is_embedded_verbatim_for_a_variety_of_inputs() {
        let mut names: Vec<String> = vec![
            "José".into(),                // accented Latin
            "Zoë Müller".into(),          // diaeresis, umlaut
            "世界".into(),                // CJK ideographs
            "こんにちは".into(),          // Japanese kana
            "Ελληνικά".into(),            // Greek
            "мир".into(),                 // Cyrillic
            "عالم".into(),                // Arabic (right-to-left)
            "עולם".into(),                // Hebrew (right-to-left)
            "🦀".into(),                  // single emoji
            "👋🏽".into(),                  // emoji + skin-tone modifier
            "👨‍👩‍👧‍👦".into(),                  // ZWJ emoji sequence (family)
            "e\u{0301}".into(),           // e + combining acute (not precomposed)
            "a\u{200B}b".into(),          // zero-width space
            "José\u{A0}S".into(),         // no-break space
            "A\nB".into(),                // embedded newline
            "\"quoted\" & {name}".into(), // format-string metacharacters
            "!!!".into(),                 // punctuation matching the suffix
            "   ".into(),                 // whitespace-only (NOT empty — still a name)
        ];
        names.push("x".repeat(10_000)); // very long name

        for name in &names {
            let out = greeting(Some(name));
            assert_eq!(out, format!("Hello, {name}!"));
            assert_eq!(out.len(), 7 + name.len() + 1, "truncation? {name:?}");
        }
    }

    // ---------------------------------------------------------------------
    // name_from_args()
    // ---------------------------------------------------------------------

    fn args(names: &[&str]) -> Vec<OsString> {
        names.iter().map(OsString::from).collect()
    }

    /// No arguments → no name.
    #[test]
    fn no_args_yields_none() {
        assert_eq!(name_from_args(&[]), None);
    }

    /// A single argument is used as-is.
    #[test]
    fn single_arg_is_used_verbatim() {
        assert_eq!(
            name_from_args(&args(&["Alambayana"])),
            Some("Alambayana".into())
        );
    }

    /// Multiple arguments are joined with a single space.
    #[test]
    fn args_are_joined_with_single_spaces() {
        assert_eq!(
            name_from_args(&args(&["Rust", "ace"])),
            Some("Rust ace".into())
        );
    }

    /// Empty-string arguments are dropped wherever they appear.
    #[test]
    fn empty_args_are_ignored_among_others() {
        assert_eq!(
            name_from_args(&args(&["", "Rust", "", "is", "great", ""])),
            Some("Rust is great".into())
        );
    }

    /// Only empty arguments → an empty (but present) name, which `greeting`
    /// turns into "Hello, there!".
    #[test]
    fn only_empty_args_yield_an_empty_name() {
        assert_eq!(name_from_args(&args(&["", ""])), Some(String::new()));
    }

    /// A whitespace-only argument is NOT empty — it is a real (odd) name.
    #[test]
    fn whitespace_only_arg_is_a_real_name() {
        assert_eq!(name_from_args(&args(&[" "])), Some(" ".into()));
    }

    /// Any non-UTF-8 argument fails the whole batch (Unix-only input). The
    /// user still *gave* input, so the result is an empty name ("Hello,
    /// there!") rather than None ("Hello, world!").
    #[test]
    #[cfg(unix)]
    fn any_invalid_utf8_arg_fails_the_batch_to_empty_name() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;
        let invalid = OsString::from(OsStr::from_bytes(&[0xFF, 0xFE, 0x41]));
        let good = OsString::from("Rust");
        // Invalid alone, first, or last — always an empty (but present) name.
        assert_eq!(
            name_from_args(std::slice::from_ref(&invalid)),
            Some(String::new())
        );
        assert_eq!(
            name_from_args(&[invalid.clone(), good.clone()]),
            Some(String::new())
        );
        assert_eq!(name_from_args(&[good, invalid]), Some(String::new()));
    }
}

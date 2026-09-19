//! `hello-world` — a tiny command-line greeting tool.
//!
//! Usage:
//! ```text
//! hello-world                    -> "Hello, world!"
//! hello-world <name>             -> "Hello, <name>!"
//! hello-world <word> <word> ...  -> "Hello, <word> <word> ...!"
//! ```
//!
//! All arguments are joined with a single space, so unquoted multi-word
//! names work the same as quoted ones: `hello-world Devajyoti Sarkar` and
//! `hello-world "Devajyoti Sarkar"` produce identical output.
//! The name is embedded **verbatim** — the program performs no escaping,
//! normalization, or truncation, and outputs valid UTF-8 as long as the
//! input argument is.
//!
//! Note: like all `std::env::args()`-based programs, an argument that is
//! not valid UTF-8 (possible on Unix) is silently skipped, so the program
//! falls back to the default greeting in that case.

/// Builds the greeting for the given name.
///
/// Returns the default greeting when `name` is `None`, otherwise greets the
/// supplied name verbatim. The logic lives in its own function (rather than
/// inline in `main`) so it can be unit-tested without spawning a process.
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

/// Entry point: collects all CLI arguments, joins them with a single space,
/// and prints the greeting.
///
/// Uses `args_os()` rather than `args()` so that a non-UTF-8 argument (only
/// possible on Unix) is treated as "no argument" instead of panicking —
/// recent Rust versions panic in `env::args()` when handed invalid UTF-8.
fn main() {
    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .filter_map(|arg| arg.into_string().ok())
        .collect();
    let name = (!args.is_empty()).then(|| args.join(" "));
    println!("{}", greeting(name.as_deref()));
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

    /// The name is embedded verbatim for a wide range of inputs: accented
    /// Latin, CJK, right-to-left scripts, emoji (including ZWJ sequences and
    /// skin-tone modifiers), combining marks, zero-width and non-breaking
    /// spaces, newlines, format-string metacharacters, and degenerate names
    /// (empty, whitespace-only, very long).
    ///
    /// The byte-length check proves there is no truncation: the output must
    /// be exactly `Hello, ` + the name's bytes + `!`.
    #[test]
    fn name_is_embedded_verbatim_for_a_variety_of_inputs() {
        let mut names: Vec<String> = vec![
            "José".into(),        // accented Latin
            "Zoë Müller".into(),  // diaeresis, umlaut
            "世界".into(),          // CJK ideographs
            "こんにちは".into(),     // Japanese kana
            "Ελληνικά".into(),    // Greek
            "мир".into(),          // Cyrillic
            "عالم".into(),          // Arabic (right-to-left)
            "עולם".into(),          // Hebrew (right-to-left)
            "🦀".into(),           // single emoji
            "👋🏽".into(),          // emoji + skin-tone modifier
            "👨‍👩‍👧‍👦".into(),      // ZWJ emoji sequence (family)
            "e\u{0301}".into(),     // e + combining acute (not precomposed)
            "a\u{200B}b".into(),    // zero-width space
            "José\u{A0}S".into(),   // no-break space
            "A\nB".into(),          // embedded newline
            "\"quoted\" & {name}".into(), // format-string metacharacters
            "!!!".into(),           // punctuation matching the suffix
            "   ".into(),           // whitespace-only
            "".into(),              // empty name
        ];
        names.push("x".repeat(10_000)); // very long name

        for name in &names {
            let out = greeting(Some(name));
            assert_eq!(out, format!("Hello, {name}!"));
            assert_eq!(out.len(), 7 + name.len() + 1, "truncation? {name:?}");
        }
    }

    /// The default greeting must be byte-for-byte the documented string.
    #[test]
    fn default_greeting_is_exactly_documented() {
        assert_eq!(greeting(None), "Hello, world!");
        assert_eq!(greeting(None).len(), 13);
    }
}

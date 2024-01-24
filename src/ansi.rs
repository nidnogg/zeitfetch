/// Truncate a string, ignoring ANSI graphics, but preserving trailing ANSI
/// graphics past the truncation point.
pub fn truncate(s: &str, n: usize) -> String {
    use unicode_segmentation::UnicodeSegmentation;

    #[derive(Debug, PartialEq, Eq)]
    enum State {
        Normal,
        Ansi,
        Final,
    }

    use State::*;

    let mut count = 0;
    let mut state = Normal;
    let mut out = String::new();

    for g in s.graphemes(true) {
        match state {
            Normal => {
                if g == "\x1b" {
                    state = Ansi;
                } else {
                    count += 1;
                    if count > n {
                        state = Final;
                        continue;
                    }
                }
            }
            Ansi => {
                if g == "m" {
                    state = Normal;
                }
            }
            Final => {
                if g != "\x1b" {
                    continue;
                }
                state = Ansi;
            }
        }
        out.push_str(g)
    }
    out.to_string()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[rstest]
    #[case("hello", 4, "hell")]
    #[case("hello", 5, "hello")]
    #[case("hello", 6, "hello")]
    fn test_truncate_plain(#[case] input: &str, #[case] n: usize, #[case] expected: &str) {
        assert_eq!(super::truncate(input, n), expected);
    }

    #[rstest]
    #[case("\x1b[1mhello\x1b[0m", 0, "\x1b[1m\x1b[0m")]
    #[case("\x1b[1mhello\x1b[0m", 4, "\x1b[1mhell\x1b[0m")]
    #[case("\x1b[1mhello\x1b[0m", 5, "\x1b[1mhello\x1b[0m")]
    #[case("\x1b[1mhello\x1b[0m", 6, "\x1b[1mhello\x1b[0m")]
    #[case("\x1b[1mhello\x1b[0m\x1b[1m", 4, "\x1b[1mhell\x1b[0m\x1b[1m")]
    #[case(
        "\x1b[1mhello\x1b[0m \x1b[1mworld!\x1b[0m",
        4,
        "\x1b[1mhell\x1b[0m\x1b[1m\x1b[0m"
    )]
    #[case(
        "\x1b[1mhello\x1b[0m \x1b[1mworld!\x1b[0m",
        7,
        "\x1b[1mhello\x1b[0m \x1b[1mw\x1b[0m"
    )]
    fn test_truncate_ansi_rstest(#[case] input: &str, #[case] n: usize, #[case] expected: &str) {
        assert_eq!(super::truncate(input, n), expected);
    }
}

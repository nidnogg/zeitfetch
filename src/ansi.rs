/// Truncate a string, ignoring ANSI graphics, but preserving trailing ANSI
/// graphics past the truncation point.
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn truncate(s: &str, n: usize) -> String {
    #[derive(Debug, PartialEq, Eq)]
    enum State {
        Normal,
        Ansi,
        Final,
    }

    use State::*;

    let mut visible_width = 0;
    let mut state = Normal;
    let mut out = String::new();
    let mut current_ansi = String::new();

    for g in s.graphemes(true) {
        match state {
            Normal => {
                if g == "\x1b" {
                    state = Ansi;
                    current_ansi.clear();
                    current_ansi.push_str(g);
                } else {
                    let char_width = g.width();
                    if visible_width + char_width > n {
                        state = Final;
                        continue;
                    }
                    visible_width += char_width;
                    out.push_str(g);
                }
            }
            Ansi => {
                current_ansi.push_str(g);
                if g == "m" {
                    out.push_str(&current_ansi);
                    state = Normal;
                }
            }
            Final => {
                if g == "\x1b" {
                    state = Ansi;
                    current_ansi.clear();
                    current_ansi.push_str(g);
                }
            }
        }
    }

    // Ensure all ANSI sequences are properly closed
    if !out.ends_with("\x1b[0m") {
        out.push_str("\x1b[0m");
    }

    out
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

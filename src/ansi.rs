use unicode_segmentation::UnicodeSegmentation;

// truncate a string ignore ANSI graphics
pub fn truncate(s: &str, n: usize) -> String {
    let mut count = 0;
    let mut out = String::new();
    let mut in_ansi = false;
    for g in s.graphemes(true) {
        if count >= n {
            break;
        }
        if g == "\x1b" {
            in_ansi = true;
        } else if g == "m" && in_ansi {
            in_ansi = false;
        } else {
            if !in_ansi {
                count += 1;
            }
        }
        out.push_str(g)
    }
    out.to_string()
}

// length of string ignoreing ANSI graphics
pub fn len(s: &str) -> usize {
    let mut count = 0;
    let mut in_ansi = false;
    for g in s.graphemes(true) {
        if g == "\x1b" {
            in_ansi = true;
        } else if g == "m" && in_ansi {
            in_ansi = false;
        } else if !in_ansi {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    #[test]
    fn test_truncate_plain() {
        assert_eq!(super::truncate("hello", 3), "hel");
        assert_eq!(super::truncate("hello", 5), "hello");
        assert_eq!(super::truncate("hello", 6), "hello");
    }

    #[test]
    fn test_truncate_ansi() {
        // TODO: ensure reset at end
        assert_eq!(super::truncate("\x1b[1mhello\x1b[0m", 3), "\x1b[1mhel");
        assert_eq!(
            super::truncate("\x1b[1mhello\x1b[0m", 5),
            "\x1b[1mhello\x1b[0m"
        );
        assert_eq!(
            super::truncate("\x1b[1mhello\x1b[0m", 6),
            "\x1b[1mhello\x1b[0m"
        );
    }
}

pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();

    let mut start: usize = 0;
    let mut end: usize = chars.len() - 1;

    while start <= end {
        let start_char;
        let end_char;

        // move to valid character
        loop {
            match chars.get(start) {
                Some(v) => {
                    if v.is_alphanumeric() {
                        start_char = *v;
                        break;
                    }

                    start = start + 1;
                }
                None => return true,
            };
        }

        // move to valid character
        loop {
            match chars.get(end) {
                Some(v) => {
                    if end <= 0 {
                        return true;
                    }

                    if v.is_alphanumeric() {
                        end_char = *v;
                        break;
                    }

                    end = end - 1;
                }
                None => return true,
            };
        }

        if start_char.to_ascii_lowercase() != end_char.to_ascii_lowercase() {
            return false;
        }

        if end <= 0 {
            return true;
        }

        start = start + 1;
        end = end - 1;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphanumeric() {
        assert_eq!(is_palindrome("Aa".into()), true);
        assert_eq!(is_palindrome("aba".into()), true);
        assert_eq!(is_palindrome("bA".into()), false);
        assert_eq!(is_palindrome("a".into()), true);
        assert_eq!(is_palindrome("2a2".into()), true);
    }

    #[test]
    fn non_alphanumeric() {
        assert_eq!(is_palindrome("{a a".into()), true);
        assert_eq!(is_palindrome("aba   ".into()), true);
        assert_eq!(is_palindrome("{10  --.0b a001".into()), false);
    }

    #[test]
    fn cases() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".into()), true);
        assert_eq!(is_palindrome("race a car".into()), false);
        assert_eq!(is_palindrome(" ".into()), true);
    }
}

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // early out
    if s_chars.len() != t_chars.len() {
        return false;
    }

    let mut char_counts: HashMap<char, i32> = HashMap::new();

    // increment hashmap for s
    for c in s_chars {
        let v = char_counts.entry(c).or_insert(0);
        *v += 1;
    }

    // decriment hashmap for t
    for c in t_chars {
        let v = char_counts.entry(c).or_insert(0);
        *v -= 1;
    }

    for (_key, value) in char_counts {
        if value != 0 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(is_anagram("aaa".to_string(), "aaa".to_string()), true);
        assert_eq!(is_anagram("aaabb".to_string(), "bbaaa".to_string()), true);
        assert_eq!(is_anagram("aaabb".to_string(), "baaa".to_string()), false);
    }

    #[test]
    fn tests() {
        assert_eq!(
            is_anagram("anagram".to_string(), "anagram".to_string()),
            true
        );
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}

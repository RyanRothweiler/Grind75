use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut note: HashMap<char, i32> = HashMap::new();

    // fill not map
    for c in magazine.chars() {
        let count = note.entry(c).or_insert(0);
        *count += 1;
    }

    // check note map
    for c in ransom_note.chars() {
        let count = note.entry(c).or_insert(0);
        *count -= 1;

        if *count < 0 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(can_construct("aa".to_string(), "aa".to_string()), true);
        assert_eq!(can_construct("a".to_string(), "aa".to_string()), true);
        assert_eq!(can_construct("aa".to_string(), "a".to_string()), false);
        assert_eq!(can_construct("ace".to_string(), "abbeec".to_string()), true);
    }
}

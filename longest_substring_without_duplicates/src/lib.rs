pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let chars: Vec<char> = s.chars().collect();

    let mut max: i32 = 1;
    let mut sub_len: i32 = 1;

    let mut left: usize = 0;
    let mut right: usize = 1;

    let mut sub_hash: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    sub_hash.insert(chars[0], 1);

    while right < chars.len() {
        if left == right {
            *sub_hash.entry(chars[left]).or_insert(0) += 1;

            sub_len = 1;
            right += 1;
            continue;
        }

        let current_valid: bool = *sub_hash.entry(chars[right]).or_insert(0) <= 0;

        // current string not valid. Move left pointer.
        if !current_valid {
            sub_len -= 1;

            *sub_hash.entry(chars[left]).or_insert(0) -= 1;
            left += 1;

            continue;
        }

        // current is valid. Keep that character and calc new max, and move right pointer
        if current_valid {
            *sub_hash.entry(chars[right]).or_insert(0) += 1;
            sub_len += 1;

            max = std::cmp::max(max, sub_len);

            right += 1;
            continue;
        }
    }

    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(length_of_longest_substring("abc".into()), 3);
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
    }
}

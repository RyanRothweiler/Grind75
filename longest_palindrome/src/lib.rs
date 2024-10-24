use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut count: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        let n = count.entry(c).or_insert(0);
        *n += 1;
    }

    let mut longest: i32 = 0;
    let mut used_odd: bool = false;

    for (_key, value) in count {
        if value % 2 == 0 {
            // event count
            longest += value;
        } else {
            // odd count

            if used_odd {
                // we've already used the odd number, so just count the evens
                longest += value - 1;
            } else {
                // one odd number is allowed because it can go in the middle
                used_odd = true;
                longest += value;
            }
        }
    }

    return longest;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(longest_palindrome("a".to_string()), 1);
        assert_eq!(longest_palindrome("aa".to_string()), 2);
        assert_eq!(longest_palindrome("aaa".to_string()), 3);
        assert_eq!(longest_palindrome("baaa".to_string()), 3);
        assert_eq!(longest_palindrome("baaab".to_string()), 5);
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    }
}

/*
2 ->
3 -> 2
5 -> 4

*/

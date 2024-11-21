pub fn map_valid(counts: &std::collections::HashMap<char, i32>, k: i32) -> bool {
    if counts.len() != 3 {
        return false;
    }

    for (_key, value) in counts {
        if *value < k {
            return false;
        }
    }

    return true;
}

pub fn take_characters(s: String, k: i32) -> i32 {
    use std::collections::HashMap;

    if k == 0 {
        return 0;
    }

    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    if !map_valid(&counts, k) {
        return -1;
    }

    let mut left: usize = 0;
    let mut right: usize = 0;

    // we want to maximize the window,
    // which will minimize the number of characters we need to take to solve
    let mut window_max: i32 = 0;

    let chs: Vec<char> = s.chars().collect();

    *counts.get_mut(&chs[0]).unwrap() -= 1;

    // println!("{:?}", counts);

    loop {
        if left == right && right == s.len() - 1 {
            break;
        }

        let is_valid = map_valid(&counts, k);
        if is_valid {
            // add one because right and left are inclusive
            window_max = window_max.max((right - left) as i32 + 1);
        }

        /*
        println!(
            "{:?} - {:?} = {:?} {:?} {:?}",
            left, right, is_valid, counts, window_max
        );
        */

        if left == right {
            right += 1;
            *counts.get_mut(&chs[right]).unwrap() -= 1;
        } else {
            // if valid then grow, else shrink
            if is_valid {
                right += 1;

                // if right is at end, and we're valid,
                // then there is no use in shrinking, that is just worse
                if right >= s.len() {
                    break;
                }

                *counts.get_mut(&chs[right]).unwrap() -= 1;
            } else {
                *counts.get_mut(&chs[left]).unwrap() += 1;
                left += 1;
            }
        }
    }

    return s.len() as i32 - window_max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let mut map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        map.entry('a').or_insert(2);
        map.entry('b').or_insert(2);
        map.entry('c').or_insert(3);

        assert_eq!(map_valid(&map, 1), true);
        assert_eq!(map_valid(&map, 2), true);
        assert_eq!(map_valid(&map, 3), false);
        assert_eq!(map_valid(&map, 4), false);
    }

    #[test]
    fn too_small() {
        assert_eq!(take_characters("a".into(), 2), -1);
        assert_eq!(take_characters("a".into(), 2), -1);
    }

    #[test]
    fn edges() {
        assert_eq!(take_characters("".into(), 2), -1);
        assert_eq!(take_characters("a".into(), 2), -1);
        assert_eq!(take_characters("a".into(), 1), -1);
    }

    #[test]
    fn first() {
        assert_eq!(take_characters("abccc".into(), 1), 3);
        // assert_eq!(take_characters("abc".into(), 1), 3);
    }

    #[test]
    fn second() {
        assert_eq!(take_characters("aabaaaacaabc".into(), 2), 8);
    }
}

fn main() {
    println!("Hello, world!");
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut set: HashSet<i32> = HashSet::new();

    // build set
    for n in nums {
        set.insert(n);
    }

    // First find the start of the sequence.
    // Then check the next num to count the sequence.
    // Then save max sequence
    let mut longest: i32 = 0;

    for n in &set {
        // length starts at 1 to include n itself
        let mut len: i32 = 1;

        // we have no left neighbor so this is the start of a sequence
        if !set.contains(&(*n - 1)) {
            let mut i = *n;

            while set.contains(&(i + 1)) {
                i += 1;
                len += 1;
            }
        }

        longest = i32::max(longest, len);
    }

    longest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(longest_consecutive(vec![100, 0, 2, 200, 3, 4, 5, 1]), 6);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
        assert_eq!(longest_consecutive(vec![1, 0, 1, 2]), 3);
        assert_eq!(longest_consecutive(vec![]), 0);
        assert_eq!(longest_consecutive(vec![1]), 1);
    }
}

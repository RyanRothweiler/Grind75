fn main() {
    println!("Hello, world!");
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for n in &nums {
        *counts.entry(*n).or_insert(0) += 1;
    }

    let mut counts_org: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

    for (key, value) in counts {
        counts_org[value as usize].push(key);
    }

    let mut ret: Vec<i32> = vec![];
    for i in (0..nums.len() + 1).rev() {
        for n in &counts_org[i] {
            ret.push(*n);
        }

        if ret.len() >= k as usize {
            break;
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        // assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}

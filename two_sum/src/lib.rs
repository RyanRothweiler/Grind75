use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let num = nums[i];

        let desired = target - num;

        if seen.contains_key(&desired) {
            // return the index
            return vec![seen[&desired], i as i32];
        } else {
            seen.insert(num, i as i32);
        }
    }

    panic!("No Solution!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let answer = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(answer, vec![0, 1]);
    }

    #[test]
    fn second() {
        let answer = two_sum(vec![3, 2, 4], 6);
        assert_eq!(answer, vec![1, 2]);
    }

    #[test]
    fn third() {
        let answer = two_sum(vec![3, 3], 6);
        assert_eq!(answer, vec![0, 1]);
    }
}

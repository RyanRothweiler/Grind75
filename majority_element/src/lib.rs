pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate_num: i32 = 0;
    let mut accum: i32 = 0;

    for i in nums {
        if accum == 0 {
            candidate_num = i;
            accum += 1;
        } else {
            if i == candidate_num {
                accum += 1;
            } else {
                accum -= 1;
            }
        }
    }

    candidate_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}

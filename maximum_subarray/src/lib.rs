pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut curr_max = nums[0];
    let mut pot_max = nums[0];

    for i in 1..nums.len() {
        let n = nums[i];
        pot_max = std::cmp::max(n, pot_max + n);

        if pot_max > curr_max {
            curr_max = pot_max;
        }
    }

    return curr_max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(max_sub_array(vec![8, -19, 5, -4, 20]), 21);
        assert_eq!(max_sub_array(vec![-1, -2]), -1);
        assert_eq!(max_sub_array(vec![1, 2]), 3);
        assert_eq!(max_sub_array(vec![1, -3, 4]), 4);
        assert_eq!(max_sub_array(vec![1, 3, 4]), 8);
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}

// Don't need both left and right array. Can just do the multiplcation in one array
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut left = vec![1; nums.len()];
    for i in 1..nums.len() {
        left[i] = nums[i - 1] * left[i - 1];
    }

    println!("{:?}", left);

    let mut right = vec![1; nums.len()];
    for i in (0..nums.len() - 1).rev() {
        right[i] = nums[i + 1] * right[i + 1];
    }

    let mut ret = vec![1; nums.len()];
    for i in 0..nums.len() {
        ret[i] = left[i] * right[i];
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}

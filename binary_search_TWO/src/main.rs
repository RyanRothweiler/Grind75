fn main() {
    println!("Hello, world!");
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;

    while left <= right {
        let mid: usize = (left + (right - left) / 2) as usize;

        if nums[mid] == target {
            return mid as i32;
        } else if target < nums[mid] {
            right = mid as i32 - 1;
        } else if target > nums[mid] {
            left = mid as i32 + 1;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
}

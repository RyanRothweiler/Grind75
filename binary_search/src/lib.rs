pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }

    if nums.len() == 1 {
        if nums[0] == target {
            return 0;
        } else {
            return -1;
        }
    }

    return run(0, nums.len() - 1, target, nums).unwrap_or(-1);
}

pub fn run(start: usize, end: usize, target: i32, nums: Vec<i32>) -> Option<i32> {
    if start == end {
        if nums[start] == target {
            return Some(start as i32);
        } else {
            return None;
        }
    }

    let len = (end - start) as f64;
    let i: usize = (len * 0.5) as usize + start;

    let mid_num = nums[i];
    if mid_num == target {
        return Some(i as i32);
    }

    if target < mid_num {
        return run(start, i, target, nums);
    } else {
        return run(i + 1, end, target, nums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(search(vec![2, 5], 0), -1);
        assert_eq!(search(vec![], 100), -1);
        assert_eq!(search(vec![0], 0), 0);
        assert_eq!(search(vec![0, 1], 0), 0);
        assert_eq!(search(vec![0, 1], 1), 1);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 100), -1);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
}

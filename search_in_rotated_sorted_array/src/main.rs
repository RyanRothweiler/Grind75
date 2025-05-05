#![allow(unused_comparisons, clippy::all)]

fn main() {
    println!("Hello, world!");
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    pub enum MoveDir {
        Unknown,
        Left,
        Right,
    }

    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;

    while (left <= right) && (left >= 0) && (right < nums.len()) {
        let mid = (left + right) / 2;

        let left_val = nums[left];
        let right_val = nums[right];
        let mid_val = nums[mid];

        let mut move_dir = MoveDir::Unknown;

        if target == mid_val {
            return mid as i32;
        } else if left_val <= mid_val {
            if target > mid_val || target < left_val {
                move_dir = MoveDir::Right;
            } else {
                move_dir = MoveDir::Left;
            }
        } else {
            if target < mid_val || target > right_val {
                move_dir = MoveDir::Left;
            } else {
                move_dir = MoveDir::Right;
            }
        }

        match move_dir {
            MoveDir::Unknown => return -1,
            MoveDir::Right => {
                left = mid + 1;
            }
            MoveDir::Left => {
                right = mid - 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(search(vec![5, 1, 2, 3, 4], 1), 1);
        assert_eq!(search(vec![5, 1, 3], 0), -1);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(search(vec![1], 0), -1);
        assert_eq!(search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
    }
}

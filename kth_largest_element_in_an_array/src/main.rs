#![allow(clippy::all, dead_code, unused_variables)]

fn main() {
    println!("Hello, world!");
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, target_index: usize) -> i32 {
        let pivot_value = nums[right];
        let mut swap_pointer = left;

        for i in left..right {
            if nums[i] <= pivot_value {
                nums.swap(i, swap_pointer);
                swap_pointer += 1;
            }
        }

        nums.swap(swap_pointer, right);
        if target_index < swap_pointer {
            return quick_select(nums, left, swap_pointer - 1, target_index);
        } else if target_index > swap_pointer {
            return quick_select(nums, swap_pointer + 1, right, target_index);
        } else {
            return pivot_value;
        }
    }

    let mut n = nums.clone();
    quick_select(&mut n, 0, nums.len() - 1, nums.len() - k as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}

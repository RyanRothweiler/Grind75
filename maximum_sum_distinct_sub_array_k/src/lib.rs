pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;

    let mut left: i32 = 0;
    let mut right: i32 = k - 1;

    let mut sum_window: i64 = 0;

    let mut occurances: HashMap<i32, i32> = HashMap::new();
    let mut duplicates_count: i32 = 0;
    let mut sum_max: i64 = 0;

    for i in 0..k {
        let num = nums[i as usize];
        sum_window += num as i64;

        *occurances.entry(num).or_insert(0) += 1;
        if *occurances.get(&num).unwrap() == 2 {
            duplicates_count += 1;
        }
    }

    if duplicates_count == 0 {
        sum_max = sum_window;
    }

    for _i in (right as usize)..(nums.len() - 1) {
        right += 1;
        let adding = nums[right as usize];
        let removing = nums[left as usize];
        left += 1;

        sum_window += adding as i64;
        sum_window -= removing as i64;

        *occurances.entry(adding).or_insert(0) += 1;
        if *occurances.get(&adding).unwrap() == 2 {
            duplicates_count += 1;
        }

        *occurances.entry(removing).or_insert(0) -= 1;
        if *occurances.get(&removing).unwrap() == 1 {
            duplicates_count -= 1;
        }

        if duplicates_count == 0 {
            sum_max = sum_max.max(sum_window);
        }
    }

    sum_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3), 15);
    }

    #[test]
    fn second() {
        assert_eq!(maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}

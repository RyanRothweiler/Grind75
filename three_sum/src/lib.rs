pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = vec![];

    if nums.len() == 0 {
        return vec![];
    }

    let mut nums = nums.clone();
    nums.sort();

    for i in 0..(nums.len() - 2) {
        // Becuase the list is sorted.
        // If the current is already greater than zero then no possible tripplet can add to 0.
        // Everything above is larger than 0
        if nums[i] > 0 {
            break;
        }

        // Move i to a new number.
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum < 0 {
                // sum is too small so increase left.
                left += 1;
            } else if sum > 0 {
                // sum is too large so decrease right.
                right -= 1;
            } else {
                // sum is exactly 0.

                // Remember the tripplet
                ret.push(vec![nums[i], nums[left], nums[right]]);

                // Increase left and right to new numbers
                while left < right && nums[left] == ret[ret.len() - 1][1] {
                    left += 1;
                }
                while left < right && nums[right] == ret[ret.len() - 1][2] {
                    right -= 1;
                }
            }
        }
    }

    ret
}

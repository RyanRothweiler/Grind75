#![allow(clippy::all)]

fn main() {
    three_sum(vec![-1, 0, 1, 2, -1, -4]);
    println!("Hello, world!");
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![];
    }

    let mut nums = nums.clone();
    nums.sort();

    let mut ret: Vec<Vec<i32>> = vec![];

    for a in 0..nums.len() {
        // skip values that we've already checked
        if a > 0 && nums[a] == nums[a - 1] {
            continue;
        }

        // run two_sum
        let mut b: usize = a + 1;
        let mut c: usize = nums.len() - 1;

        while b < c {
            let sum = nums[a] + nums[b] + nums[c];
            if sum == 0 {
                ret.push(vec![nums[a], nums[b], nums[c]]);

                // move left to new number
                b += 1;
                while b < c && nums[b] == nums[b - 1] {
                    b += 1;
                }
            } else if sum > 0 {
                c -= 1;
            } else if sum < 0 {
                b += 1;
            }
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}

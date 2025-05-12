#![allow(clippy::all)]

fn main() {
    println!("Hello, world!");
}

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;

    let mut i = 0;
    while i <= right {
        match nums[i] {
            0 => {
                nums.swap(i, left);
                left += 1;
                i += 1;
            }
            1 => {
                // do nothing. Ones go in the middle.
                i += 1;
            }
            2 => {
                // Don't incrment i
                nums.swap(i, right);

                if right == 0 {
                    return;
                }

                right -= 1;
            }
            _ => panic!("Invalid input"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn smaller() {
        let mut v = vec![2, 0, 1];
        sort_colors(&mut v);
        assert_eq!(v, vec![0, 1, 2]);
    }

    #[test]
    fn single() {
        let mut v = vec![2];
        sort_colors(&mut v);
        assert_eq!(v, vec![2]);
    }

    #[test]
    fn twos() {
        let mut v = vec![2, 2];
        sort_colors(&mut v);
        assert_eq!(v, vec![2, 2]);
    }
}

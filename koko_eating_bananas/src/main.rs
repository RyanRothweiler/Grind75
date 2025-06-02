fn main() {
    println!("Hello, world!");
}

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let max: i64 = *piles.iter().max().unwrap() as i64;

    let mut left: i64 = 0;
    let mut right: i64 = max;

    while left <= right {
        let mid = left + (right - left) / 2;

        if valid_rate(&piles, mid, h) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}

pub fn valid_rate(piles: &Vec<i32>, rate: i64, hours_avail: i32) -> bool {
    let mut hours_needed: i64 = 0;

    for p in piles {
        hours_needed += (*p as f64 / rate as f64).ceil() as i64;
        if hours_needed > hours_avail as i64 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_rate() {
        assert!(!super::valid_rate(&vec![3, 6, 7, 11], 1, 11));
        assert!(super::valid_rate(&vec![3, 6, 7, 11], 100, 11));
        assert!(super::valid_rate(&vec![3, 6, 7, 11], 3, 11));

        assert!(!super::valid_rate(&vec![3, 6, 7, 11], 3, 8));
        assert!(super::valid_rate(&vec![3, 6, 7, 11], 4, 8));
        assert!(super::valid_rate(&vec![3, 6, 7, 11], 5, 8));
    }

    #[test]
    fn koko_eating() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }
}

fn main() {
    println!("Hello, world!");
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut counts: Vec<i32> = vec![i32::MAX; amount as usize + 1];
    counts[0] = 0;

    for i in 1..(amount + 1) {
        for c in &coins {
            let attempt_amount = i - c;
            if attempt_amount >= 0 {
                counts[i as usize] = i32::min(
                    counts[i as usize],
                    counts[attempt_amount as usize].saturating_add(1),
                );
            }
        }
    }

    if counts[amount as usize] == i32::MAX {
        -1
    } else {
        counts[amount as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }
}

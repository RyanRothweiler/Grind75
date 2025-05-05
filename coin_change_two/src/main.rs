use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    fn check(
        current: i32,
        cut: usize,
        target: i32,
        coins: &Vec<i32>,
        memo: &mut HashMap<(i32, usize), i32>,
    ) -> i32 {
        if current > target {
            return 0;
        }

        if current == target {
            return 1;
        }

        if cut >= coins.len() {
            return 0;
        }

        if let Some(e) = memo.get(&(current, cut)) {
            return *e;
        }

        let val = check(current + coins[cut], cut, target, coins, memo)
            + check(current, cut + 1, target, coins, memo);

        memo.insert((current, cut), val);
        val
    }

    let mut memo: HashMap<(i32, usize), i32> = HashMap::new();

    check(0, 0, amount, &coins, &mut memo)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first() {
        assert_eq!(change(5, vec![1, 2, 5]), 4);
        assert_eq!(change(3, vec![2]), 0);
        assert_eq!(change(10, vec![10]), 1);
        assert_eq!(change(0, vec![7]), 1);
    }
}

use std::collections::HashMap;

pub fn climb_stairs(n: i32) -> i32 {
    fn check(treads_remaining: i32, mut memo: &mut HashMap<i32, i32>) -> i32 {
        if treads_remaining < 0 {
            return 0;
        }

        if treads_remaining == 0 {
            return 1;
        }

        if memo.contains_key(&treads_remaining) {
            return *memo.get(&treads_remaining).unwrap();
        }

        let res = check(treads_remaining - 1, &mut memo) + check(treads_remaining - 2, &mut memo);
        memo.insert(treads_remaining, res);

        return res;
    }

    let mut memo: HashMap<i32, i32> = HashMap::new();

    return check(n - 1, &mut memo) + check(n - 2, &mut memo);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
    }
}

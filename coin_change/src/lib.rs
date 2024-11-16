pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    fn check(
        coins: &Vec<i32>,
        amount: i32,
        memo_best: &mut std::collections::HashMap<i32, i32>,
    ) -> i32 {
        // check if count for the amount is known
        match memo_best.get(&amount) {
            Some(a) => {
                return *a;
            }
            None => {}
        }

        if amount == 0 {
            return 0;
        }

        let mut ans: i32 = i32::MAX;

        for c in coins {
            let rem = amount - c;
            if rem >= 0 {
                let sub_ans = check(coins, rem, memo_best);
                if sub_ans != i32::MAX {
                    ans = ans.min(sub_ans + 1);
                }
            }
        }

        memo_best.insert(amount, ans);
        return ans;
    }

    let c = coins.clone();
    let mut memo: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    let r = check(&c, amount, &mut memo);
    if r == i32::MAX {
        return -1;
    } else {
        return r;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }
}

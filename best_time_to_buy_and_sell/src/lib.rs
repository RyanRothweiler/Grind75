pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit: i32 = 0;
    let mut window_min: i32 = match prices.get(0) {
        Some(v) => *v,
        None => return 0,
    };

    // skip the first index, window_min holds that
    for price in prices.iter().skip(1) {
        if *price < window_min {
            window_min = *price;
        } else {
            max_profit = std::cmp::max(max_profit, *price - window_min);
        }
    }

    return max_profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors() {
        assert_eq!(max_profit(vec![]), 0);
        assert_eq!(max_profit(vec![0]), 0);
        assert_eq!(max_profit(vec![1, 0]), 0);
    }

    #[test]
    fn lists() {
        assert_eq!(max_profit(vec![0, 1]), 1);
        assert_eq!(max_profit(vec![0, 1, 10]), 10);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 10, 1, 2, 20]), 19);
    }
}

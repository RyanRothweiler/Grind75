fn main() {
    println!("Hello, world!");
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit: i32 = 0;

    if prices.is_empty() {
        return max_profit;
    }

    let mut p_min = prices[0];

    for p in prices {
        p_min = i32::min(p_min, p);
        max_profit = i32::max(max_profit, p - p_min);
    }

    max_profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {}
}

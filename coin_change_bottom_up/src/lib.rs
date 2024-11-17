pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut counts = vec![i32::MAX, amount + 1];
    let amount = amount as usize;

    counts[0] = 0;

    for am in 1..amount {
        for c in &coins {
            if am as i32 - c >= 0 {
                counts[am] = counts[am].min(counts[(am as i32 - c) as usize].saturating_add(1));
            }
        }
    }

    if counts[amount] == i32::MAX {
        return -1;
    } else {
        return counts[amount];
    }
}

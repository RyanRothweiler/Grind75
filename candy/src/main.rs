fn main() {
    candy(vec![1, 0, 2]);
}

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candies: Vec<i32> = vec![1; ratings.len()];

    // check left neighbor starting at left side.
    for (i, r) in ratings.iter().enumerate().skip(1) {
        if *r > ratings[i - 1] && candies[i] <= candies[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    // check right neighbor starting at left side.
    for (i, r) in ratings.iter().enumerate().rev().skip(1) {
        if *r > ratings[i + 1] && candies[i] <= candies[i + 1] {
            candies[i] = candies[i + 1] + 1;
        }
    }

    let mut accum: i32 = 0;
    for c in candies {
        accum += c;
    }

    accum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn working() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
    }
}

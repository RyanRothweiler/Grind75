fn main() {
    println!("Hello, world!");
}

pub fn trap(height: Vec<i32>) -> i32 {
    let mut max_left: Vec<i32> = vec![0; height.len()];
    let mut max_right: Vec<i32> = vec![0; height.len()];

    let mut max: i32 = height[0];
    for (i, h) in height.iter().enumerate().skip(1) {
        max_left[i] = max;
        max = i32::max(max, *h);
    }

    let mut max: i32 = height[height.len() - 1];
    for (i, h) in height.iter().enumerate().rev().skip(1) {
        max_right[i] = max;
        max = i32::max(max, *h);
    }

    let mut accum: i32 = 0;
    for (i, h) in height.iter().enumerate() {
        let wat = i32::min(max_left[i], max_right[i]) - h;
        if wat > 0 {
            accum += wat;
        }
    }

    accum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}

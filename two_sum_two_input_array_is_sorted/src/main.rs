fn main() {
    println!("Hello, world!");
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut a: usize = 0;
    let mut b: usize = numbers.len() - 1;

    while a < b {
        let sum = numbers[a] + numbers[b];

        if sum == target {
            return vec![a as i32 + 1, b as i32 + 1];
        } else if sum < target {
            a += 1;
        } else if sum > target {
            b -= 1;
        }
    }

    panic!("Invalid. Solution is guaranteed");
}

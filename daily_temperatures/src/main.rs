fn main() {
    println!("Hello, world!");
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(i32, usize)> = vec![];
    let mut sol: Vec<i32> = vec![0; temperatures.len()];

    for (i, temp) in temperatures.iter().enumerate() {
        loop {
            if let Some(last) = stack.last() {
                if *temp > last.0 {
                    sol[last.1] = (i - last.1) as i32;
                    stack.pop();
                } else {
                    stack.push((*temp, i));
                    break;
                }
            } else {
                stack.push((*temp, i));
                break;
            }
        }
    }

    sol
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}

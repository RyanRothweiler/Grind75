pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    for st in tokens {
        match st.as_str() {
            "+" => {
                let first: i32 = stack.pop().unwrap();
                let second: i32 = stack.pop().unwrap();

                stack.push(second + first);
            }
            "-" => {
                let first: i32 = stack.pop().unwrap();
                let second: i32 = stack.pop().unwrap();

                stack.push(second - first);
            }
            "/" => {
                let first: i32 = stack.pop().unwrap();
                let second: i32 = stack.pop().unwrap();

                stack.push(second / first);
            }
            "*" => {
                let first: i32 = stack.pop().unwrap();
                let second: i32 = stack.pop().unwrap();

                stack.push(second * first);
            }
            _ => {
                // if not operator then assume value
                let val = st.parse::<i32>().unwrap();

                stack.push(val);
            }
        }
    }

    return stack.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(eval_rpn(vec!["3".into(), "4".into(), "+".into()]), 7);
    }
}

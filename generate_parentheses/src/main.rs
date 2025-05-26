fn main() {
    println!("Hello, world!");
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn try_add(curr: String, open_count: i32, closed_count: i32, n: i32, sol: &mut Vec<String>) {
        if open_count == n && closed_count == n {
            println!("adding {:?}", curr);
            sol.push(curr);
        } else {
            // allowed to add a close if we have more open
            if open_count > closed_count {
                let mut new_curr = curr.clone();
                new_curr.push(')');
                try_add(new_curr, open_count, closed_count + 1, n, sol);
            }

            // allowed to add an open if we have some to use
            if open_count < n {
                let mut new_curr = curr.clone();
                new_curr.push('(');
                try_add(new_curr, open_count + 1, closed_count, n, sol);
            }
        }
    }

    let mut sol: Vec<String> = vec![];
    try_add("".into(), 0, 0, n, &mut sol);
    sol
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(generate_parenthesis(1), vec!["()"]);
    }
}

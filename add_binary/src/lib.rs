// this is like O(6n) or something like that. Optimizations could be made for sure.
pub fn add_binary(a: String, b: String) -> String {
    // iterate through the longer one. first is longest
    let first: String;
    let second: String;
    if a.len() > b.len() {
        first = a;
        second = b;
    } else {
        first = b;
        second = a;
    }

    let first_chars: Vec<char> = first.chars().rev().collect();
    let second_chars: Vec<char> = second.chars().rev().collect();

    let mut accum: String = String::new();

    let mut carried: bool = false;

    for n in 0..first_chars.len() {
        let fc: char = *first_chars.get(n).unwrap_or(&'0');
        let sc: char = *second_chars.get(n).unwrap_or(&'0');

        match (fc, sc) {
            ('0', '0') => {
                // if not carried then push 1 and don't carry.
                if carried {
                    accum.push('1')
                } else {
                    accum.push('0');
                }
                carried = false;
            }
            ('1', '0') | ('0', '1') => {
                if carried {
                    accum.push('0');
                } else {
                    accum.push('1');
                }
            }
            ('1', '1') => {
                if carried {
                    accum.push('1');
                } else {
                    accum.push('0');
                }

                carried = true;
            }
            _ => {
                panic!("Invalid input strings");
            }
        }

        println!("{:?} : {fc}, {sc} -> {accum}", n);
    }

    if carried {
        accum.push('1');
    }

    return accum.chars().rev().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            add_binary("1".to_string(), "0".to_string()),
            "1".to_string()
        );

        assert_eq!(
            add_binary("0".to_string(), "1".to_string()),
            "1".to_string()
        );

        assert_eq!(
            add_binary("1".to_string(), "1".to_string()),
            "10".to_string()
        );

        assert_eq!(
            add_binary("10".to_string(), "1".to_string()),
            "11".to_string()
        );

        assert_eq!(
            add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );

        assert_eq!(
            add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}

#![allow(dead_code)]

use std::num::ParseIntError;

#[derive(Debug)]
enum StringDecodeError {
    InvalidInput(String),
    ParseIntError { error: ParseIntError, input: String },
}

fn main() {
    println!("Hello, world!");
}

const SENTINEL: char = '#';

fn encode(input: &Vec<String>) -> String {
    let mut ret = String::new();

    for i in input {
        let c: String = format!("{:?}", i.len());
        ret.push_str(&c);
        ret.push(SENTINEL);
        ret.push_str(i);
    }

    ret
}

fn decode(input: String) -> Result<Vec<String>, StringDecodeError> {
    let mut ret: Vec<String> = vec![];

    let chars: Vec<char> = input.chars().collect();

    let mut i: usize = 0;
    loop {
        let start: usize = i;

        while chars[i] != SENTINEL {
            println!("{:?}", chars[i]);

            i += 1;

            if i >= chars.len() {
                return Err(StringDecodeError::InvalidInput(input.clone()));
            }
        }

        let len_str = &input[start..i];
        let len: usize =
            len_str
                .parse()
                .map_err(|input_error| StringDecodeError::ParseIntError {
                    error: input_error,
                    input: len_str.into(),
                })?;

        // Move past the sentinel value
        i += 1;

        let str_val: String = input[i..(i + len)].into();
        ret.push(str_val);

        i += len;

        if i >= chars.len() {
            break;
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        let i: Vec<String> = vec![
            "thisoneisreally longthisoneisreally long####&FJKEIODK".into(),
            "first".into(),
            "second".into(),
            "third".into(),
            "111.233#1001###11".into(),
        ];

        let encoded = encode(&i);

        let decoded = decode(encoded).unwrap();

        assert_eq!(i, decoded);
    }
}

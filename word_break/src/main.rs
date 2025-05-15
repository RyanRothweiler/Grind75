#![allow(clippy::all, unused_variables, unused_mut)]

fn main() {
    println!("Hello, world!");
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let s_chars: Vec<char> = s.chars().collect();

    let mut dp: Vec<bool> = vec![false; s.len() + 1];

    dp[s.len()] = true;

    for i in (0..s.len()).rev() {
        // let s_target: String = s.split_at(i).1.into();

        for wd in &word_dict {
            let word_checking_chars: Vec<char> = wd.chars().collect();
            // let mut i_checking = i;

            // check if this word matches
            let mut valid = true;
            for wdi in 0..wd.len() {
                let s_index = i + wdi;
                if s_index >= s.len() {
                    valid = false;
                    break;
                }

                // check this word and then word of what is remaining
                let word_char = word_checking_chars[wdi];
                let s_char = s_chars[s_index];
                if word_char != s_char {
                    valid = false;
                    break;
                }
            }

            if valid {
                let i_remain = i + wd.len();
                if valid && dp[i_remain] {
                    dp[i] = true;
                    break;
                }
            }
        }
    }

    return dp[0];
}

#[cfg(test)]
mod test {
    use super::*;

    /*
    #[test]
    fn working() {
        assert_eq!(
            word_break("leetcode".into(), vec!["leet".into(), "code".into()]),
            true
        );
    }
    */

    #[test]
    fn cars() {
        assert_eq!(
            word_break("cars".into(), vec!["car".into(), "ca".into(), "rs".into()]),
            true
        );
    }
}

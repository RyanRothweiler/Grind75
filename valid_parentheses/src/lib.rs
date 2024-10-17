#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    IsValidInvalidString,
}

pub fn is_valid(s: String) -> Result<bool, Error> {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            // just push opening character onto stack
            '(' | '[' | '{' => stack.push(c),

            // check closing characters
            ')' => {
                if !top_char_correct('(', &mut stack) {
                    return Ok(false);
                }
            }
            ']' => {
                if !top_char_correct('[', &mut stack) {
                    return Ok(false);
                }
            }
            '}' => {
                if !top_char_correct('{', &mut stack) {
                    return Ok(false);
                }
            }
            _ => return Err(Error::IsValidInvalidString),
        }
    }

    // Stack should be empty at the end.
    // If not then there were un-matched parens
    return Ok(stack.len() == 0);
}

pub fn top_char_correct(ch: char, stack: &mut Vec<char>) -> bool {
    match stack.pop() {
        Some(top) => {
            return top == ch;
        }
        None => {
            // More closing characters than there are opening
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid() {
        assert_eq!(
            is_valid("aesddfkljeo[]".into()),
            Err(Error::IsValidInvalidString)
        );
    }

    #[test]
    fn simple() {
        assert_eq!(is_valid("()".into()), Ok(true));
        assert_eq!(is_valid("[]".into()), Ok(true));
        assert_eq!(is_valid("{}".into()), Ok(true));
        assert_eq!(is_valid("{".into()), Ok(false));
    }

    #[test]
    fn comlex() {
        assert_eq!(is_valid("()()()".into()), Ok(true));
        assert_eq!(is_valid("(())".into()), Ok(true));
        assert_eq!(is_valid("([{}])".into()), Ok(true));
        assert_eq!(is_valid("([{})".into()), Ok(false));
    }
}

pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    fn circular(i: i32, len: i32) -> usize {
        return i.rem_euclid(len) as usize;
    }

    fn get_circular(code: &Vec<i32>, i: i32) -> i32 {
        return code[circular(i, code.len() as i32)];
    }

    let mut ans: Vec<i32> = vec![0; code.len()];

    if k == 0 {
        return ans;
    }

    let mut left: i32 = 1;
    let mut right: i32 = k.abs();

    let mut window: i32 = 0;

    //set window
    for i in 0..k.abs() {
        window += get_circular(&code, i + 1);
    }

    for _i in 0..code.len() {
        if k > 0 {
            ans[circular(left - 1, code.len() as i32)] = window;
        } else {
            ans[circular(right + 1, code.len() as i32)] = window;
        }

        right += 1;

        let adding = get_circular(&code, right);
        let removing = get_circular(&code, left);

        window += adding;
        window -= removing;

        left += 1;
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(decrypt(vec![1, 2, 3, 4], 0), vec![0; 4]);
        assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }

    #[test]
    fn second() {
        assert_eq!(
            decrypt(vec![10, 5, 7, 7, 3, 2, 10, 3, 6, 9, 1, 6], -4),
            vec![22, 26, 22, 28, 29, 22, 19, 22, 18, 21, 28, 19]
        );
    }
}

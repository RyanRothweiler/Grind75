pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    fn circular(code: &Vec<i32>, i: i32) -> i32 {
        // println!("{:?}", i);

        let md = i.rem_euclid(code.len() as i32) as usize;
        // println!("{:?}", md);

        return code[md];
    }

    let mut ans: Vec<i32> = vec![0; code.len()];

    if k == 0 {
        return ans;
    }

    let mut dir: i32 = 1;
    if k < 0 {
        dir = -1;
    }

    let mut left: i32 = dir;
    let mut right: i32 = k + dir;

    let k = k.abs();

    let mut window: i32 = 0;

    //set window
    for i in 0..k {
        window += circular(&code, (i + 1) * dir);
    }

    if dir < 0 {
        left -= 1;
        right -= 1;
    }

    println!("left{:?}, right{:?}", left, right);

    for i in 0..code.len() {
        ans[i as usize] = window;

        let adding = circular(&code, right);
        let removing = circular(&code, left);

        window += adding;
        window -= removing;

        println!(
            "adding {:?}({:?}) removing {:?}({:?}) window {:?}",
            adding, right, removing, left, window
        );

        right += 1;
        left += 1;

        // break;
    }

    println!("{:?}", window);
    // ans[k as usize] = window;

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        // assert_eq!(decrypt(vec![1, 2, 3, 4], 0), vec![0; 4]);
        // assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
        // panic!("");
    }

    #[test]
    fn second() {
        assert_eq!(
            decrypt(vec![10, 5, 7, 7, 3, 2, 10, 3, 6, 9, 1, 6], -4),
            vec![22, 26, 22, 28, 29, 22, 19, 22, 18, 21, 28, 19]
        );
    }
}

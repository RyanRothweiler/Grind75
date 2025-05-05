fn main() {
    println!("Hello, world!");
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn search(
        cut: usize,
        current_sum: i32,
        sum_list: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        all_valids: &mut Vec<Vec<i32>>,
    ) {
        if cut >= candidates.len() {
            return;
        }

        if current_sum > target {
            return;
        }

        if current_sum == target {
            all_valids.push(sum_list.clone());
            return;
        }

        sum_list.push(candidates[cut]);
        search(
            cut,
            current_sum + candidates[cut],
            sum_list,
            candidates,
            target,
            all_valids,
        );

        sum_list.pop();

        search(
            cut + 1,
            current_sum,
            sum_list,
            candidates,
            target,
            all_valids,
        );
    }

    let mut all_valids: Vec<Vec<i32>> = vec![];

    search(0, 0, &mut vec![], &candidates, target, &mut all_valids);

    all_valids
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first() {
        let ret = combination_sum(vec![2, 3, 6, 7], 7);
        assert_eq!(ret, vec![vec![2, 2, 3], vec![7]]);
    }
}

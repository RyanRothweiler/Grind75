#![allow(clippy::all, unused_variables)]

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations: Vec<Vec<i32>> = vec![vec![]];

    for new_num in nums {
        let mut new_perms: Vec<Vec<i32>> = vec![];
        for p in &permutations {
            for i in 0..(p.len() + 1) {
                let mut new_perm = p.clone();
                new_perm.insert(i, new_num);
                new_perms.push(new_perm);
            }
        }

        permutations = new_perms;
    }

    return permutations;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        permute(vec![1, 2, 3]);
    }
}

#![allow(unused_mut, unused_variables, clippy::all)]

fn main() {
    println!("Hello, world!");
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut ret: Vec<Vec<String>> = vec![];
    let mut group_ids: HashMap<[u8; 128], usize> = HashMap::new();

    for st in strs {
        let mut ana: [u8; 128] = [0; 128];
        for c in st.chars() {
            let c_ascii: usize = c.to_ascii_lowercase() as usize;

            ana[c_ascii] += 1;
        }

        if group_ids.contains_key(&ana) {
            let group_id: usize = *group_ids.get(&ana).unwrap();
            ret[group_id].push(st.clone());
        } else {
            let id = ret.len();

            ret.push(vec![]);
            ret[id].push(st.clone());

            group_ids.insert(ana, id);
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(
            group_anagrams(vec![
                "eat".into(),
                "tea".into(),
                "tan".into(),
                "ate".into(),
                "nat".into(),
                "bat".into()
            ]),
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );
    }
}

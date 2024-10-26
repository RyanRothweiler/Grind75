use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        if map.contains_key(&n) {
            return true;
        } else {
            map.insert(n, 1);
        }
    }

    return false;
}

#![allow(clippy::all, unused_variables, dead_code)]

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store
            .entry(key)
            .or_insert(vec![])
            .push((timestamp, value))
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(list) = self.store.get(&key) {
            // That entry exists. Use binary search to find the entry.
            // If we don't find the exact timestamp then we return the previous timestamp
            // We can use the std vec binary search for this but I'm going to write it out myself for practice

            if list[0].0 > timestamp {
                return "".into();
            }

            let mut left: usize = 0;
            let mut right: usize = list.len() - 1;

            while left < right {
                let mid: usize = (left + right) / 2;
                let mid_val = list[mid].0;

                if timestamp == mid_val {
                    return list[mid].1.clone();
                } else if timestamp < mid_val {
                    right = mid - 1;
                } else if timestamp > mid_val {
                    left = mid + 1;
                }
            }

            // Didn't find the exact value, so we just send the nearest lower

            if timestamp >= list[left].0 {
                return list[left].1.clone();
            }

            return list[left - 1].1.clone();
        } else {
            return "".into();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set() {
        let mut tm = TimeMap::new();
        tm.set("first key".into(), "cats".into(), 0);
        tm.set("first key".into(), "cats".into(), 1);
    }

    #[test]
    fn get() {
        let mut tm = TimeMap::new();

        assert_eq!(tm.get("invalid key".into(), 0), "");

        tm.set("foo".into(), "bar".into(), 1);

        assert_eq!(tm.get("foo".into(), 1), "bar");
        assert_eq!(tm.get("foo".into(), 3), "bar");

        tm.set("foo".into(), "dog".into(), 2);
        assert_eq!(tm.get("foo".into(), 3), "dog");

        tm.set("foo".into(), "third".into(), 2);
        assert_eq!(tm.get("foo".into(), 3), "third");

        tm.set("foo".into(), "big".into(), 10);
        assert_eq!(tm.get("foo".into(), 9), "third");
    }

    #[test]
    fn one() {
        let mut tm = TimeMap::new();

        tm.set("love".into(), "high".into(), 10);
        tm.set("love".into(), "low".into(), 20);

        assert_eq!(tm.get("love".into(), 5), "");
        assert_eq!(tm.get("love".into(), 10), "high");
        assert_eq!(tm.get("love".into(), 15), "high");
        assert_eq!(tm.get("love".into(), 20), "low");
        assert_eq!(tm.get("love".into(), 25), "low");
    }

    #[test]
    fn fifty_two() {
        let mut tm = TimeMap::new();

        tm.set("assem".into(), "fat".into(), 20);
        tm.set("assem".into(), "thin".into(), 40);

        tm.set("hamza".into(), "thin".into(), 41);

        assert_eq!(tm.get("hamza".into(), 20), "");
    }
}

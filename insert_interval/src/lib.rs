#![allow(unused_variables, dead_code)]

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Interval {
    pub start: i32,
    pub end: i32,
}

impl From<Vec<i32>> for Interval {
    fn from(input: Vec<i32>) -> Self {
        // TODO if in production then I'd do some validation here

        Self {
            start: input[0],
            end: input[1],
        }
    }
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    fn overlaps(x: Interval, y: Interval) -> bool {
        // one ends before the other starts
        if x.end < y.start {
            return false;
        }
        if y.end < x.start {
            return false;
        }

        // both intervals have the same start
        if x.start == y.start {
            return true;
        }

        // end and start are the same, thus overlap
        if x.end == y.start {
            return true;
        }
        if y.end == x.start {
            return true;
        }

        // one is within the other
        if x.start <= y.start && x.end >= y.end {
            return true;
        }
        if y.start <= x.start && y.end >= x.end {
            return true;
        }

        // ends overlap
        if x.start <= y.start && x.end <= y.end {
            return true;
        }
        if y.start <= x.start && y.end <= x.end {
            return true;
        }

        panic!("All cases should be handled");
    }

    fn combine(x: Self, y: Self) -> Self {
        if !Self::overlaps(x.clone(), y.clone()) {
            panic!("Must overlap!");
        }

        Self {
            start: std::cmp::min(x.start, y.start),
            end: std::cmp::max(x.end, y.end),
        }
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![self.start, self.end]
    }
}

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    fn binary_combine(
        ints: &Vec<Vec<i32>>,
        inserting: Interval,
        start: usize,
        end: usize,
    ) -> Vec<Vec<i32>> {
        // println!("{:?} <> {:?}", start, end);

        // end and start pass or equal, then there is nothing to combine with,
        // so we need to just insert the new interval here
        if start > end {
            let mut new_ray = ints.clone();
            new_ray.insert(start, inserting.to_vec());
            return new_ray;
        }

        let mid_index: usize = (start as f32 + ((end - start) as f32 * 0.5)) as usize;
        let mid_interval: Interval = Interval::from(ints[mid_index].clone());

        if Interval::overlaps(mid_interval, inserting) {
            // found place to insert, so combine and insert.

            let mut new_ray = ints.clone();
            // new_ray.remove(mid_index);

            let mut new_combined = Interval::combine(mid_interval, inserting);

            // combine left until we can't combine or hit end
            let mut final_insert: usize = mid_index;
            let mut did_combine = false;

            // println!("{:?} {:?} {:?}", new_ray, new_combined, final_insert);

            loop {
                if final_insert >= 0 && final_insert < new_ray.len() {
                    let checking_interval = Interval::from(new_ray[final_insert].clone());
                    if Interval::overlaps(new_combined, checking_interval) {
                        did_combine = true;

                        new_combined = Interval::combine(new_combined, checking_interval);

                        /*
                        println!(
                            "ate left {:?} {:?}         {:?}",
                            new_ray, final_insert, new_combined
                        );
                        */
                        new_ray.remove(final_insert);

                        if final_insert == 0 {
                            break;
                        } else {
                            final_insert -= 1;
                        }
                    } else {
                        if did_combine {
                            final_insert += 1;
                        }
                        break;
                    }
                } else {
                    break;
                }
            }

            // combine right until we can't combine or hit end
            loop {
                if final_insert < new_ray.len() {
                    let checking_interval = Interval::from(new_ray[final_insert].clone());
                    if Interval::overlaps(new_combined, checking_interval) {
                        new_combined = Interval::combine(new_combined, checking_interval);
                        new_ray.remove(final_insert);

                        // println!("ate right {:?} {:?}", new_ray, final_insert);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // println!("inserting combined {:?} {:?}", new_ray, final_insert);
            new_ray.insert(final_insert, new_combined.to_vec());

            // TODO check if the new interval can combine with multiple
            // continue checking until can't combine

            return new_ray;
        } else if inserting.end < mid_interval.start {
            // if were at the very left of the array, then insert at start
            if mid_index == 0 {
                let mut new_ray = ints.clone();
                new_ray.insert(0, inserting.to_vec());
                return new_ray;
            }

            // binary search down left
            return binary_combine(ints, inserting, start, mid_index - 1);
        } else if inserting.start > mid_interval.end {
            // binary search down right
            return binary_combine(ints, inserting, mid_index + 1, end);
        }

        panic!("Should never happen");
    }

    if intervals.len() == 0 {
        return vec![new_interval];
    }

    return binary_combine(&intervals, new_interval.into(), 0, intervals.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_combine_empty() {
        let list: Vec<Vec<i32>> = vec![];
        let ins: Vec<i32> = vec![2, 3];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![2, 3]], ret);
    }

    #[test]
    fn insert_entire_into() {
        let list: Vec<Vec<i32>> = vec![vec![1, 5]];
        let ins: Vec<i32> = vec![2, 3];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![1, 5]], ret);
    }

    #[test]
    fn insert_combine_entire_mid() {
        let list: Vec<Vec<i32>> = vec![vec![0, 1], vec![2, 5], vec![10, 20]];
        let ins: Vec<i32> = vec![2, 3];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(list, ret);
    }

    #[test]
    fn insert_combine_even() {
        let list: Vec<Vec<i32>> = vec![vec![0, 1], vec![2, 5], vec![8, 9], vec![10, 20]];
        let ins: Vec<i32> = vec![2, 3];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(list, ret);
    }

    #[test]
    fn insert_combine_even_right() {
        let list: Vec<Vec<i32>> = vec![vec![0, 1], vec![2, 5], vec![8, 10], vec![20, 30]];
        let ins: Vec<i32> = vec![10, 11];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![0, 1], vec![2, 5], vec![8, 11], vec![20, 30]], ret);
    }

    #[test]
    fn insert_combine_half_mid() {
        let list: Vec<Vec<i32>> = vec![vec![0, 1], vec![3, 5], vec![10, 20]];
        let ins: Vec<i32> = vec![2, 3];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![0, 1], vec![2, 5], vec![10, 20]], ret);
    }

    #[test]
    fn insert_combine_start() {
        let list: Vec<Vec<i32>> = vec![vec![0, 1], vec![3, 5], vec![10, 20]];
        let ins: Vec<i32> = vec![-1, 2];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![-1, 2], vec![3, 5], vec![10, 20]], ret);
    }

    #[test]
    fn insert_in_mid() {
        let list: Vec<Vec<i32>> = vec![vec![-1, 0], vec![3, 5], vec![10, 20]];
        let ins: Vec<i32> = vec![1, 2];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![-1, 0], vec![1, 2], vec![3, 5], vec![10, 20]], ret);
    }

    #[test]
    fn insert_to_start() {
        let list: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 5], vec![10, 20]];
        let ins: Vec<i32> = vec![-1, 0];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![-1, 0], vec![1, 2], vec![3, 5], vec![10, 20]], ret);
    }

    #[test]
    fn insert_to_end() {
        let list: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 5], vec![10, 20]];
        let ins: Vec<i32> = vec![100, 200];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(
            vec![vec![1, 2], vec![3, 5], vec![10, 20], vec![100, 200]],
            ret
        );
    }

    #[test]
    fn insert_multiple() {
        let list: Vec<Vec<i32>> = vec![vec![1, 3], vec![6, 9]];
        let ins: Vec<i32> = vec![0, 10];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![0, 10]], ret);
    }

    #[test]
    fn insert_combine_some() {
        let list: Vec<Vec<i32>> = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let ins: Vec<i32> = vec![4, 8];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![1, 2], vec![3, 10], vec![12, 16]], ret);
    }

    #[test]
    fn insert_consume() {
        let list: Vec<Vec<i32>> = vec![vec![2, 5], vec![6, 7], vec![8, 9]];
        let ins: Vec<i32> = vec![0, 10];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![0, 10]], ret);
    }

    #[test]
    fn insert_more_consume() {
        let list: Vec<Vec<i32>> = vec![vec![2, 4], vec![5, 7], vec![8, 10], vec![11, 13]];
        let ins: Vec<i32> = vec![3, 6];
        let ret = super::insert(list.clone(), ins);

        assert_eq!(vec![vec![2, 7], vec![8, 10], vec![11, 13]], ret);
    }

    #[test]
    fn overlaps() {
        assert_eq!(
            Interval::overlaps(Interval::new(1, 2), Interval::new(3, 4)),
            false
        );

        assert_eq!(
            Interval::overlaps(Interval::new(1, 2), Interval::new(-1, 0)),
            false
        );

        assert_eq!(
            Interval::overlaps(Interval::new(3, 4), Interval::new(1, 2)),
            false
        );

        assert_eq!(
            Interval::overlaps(Interval::new(1, 2), Interval::new(2, 3)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(3, 4), Interval::new(4, 5)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(2, 3), Interval::new(1, 2)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(1, 2), Interval::new(1, 2)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(1, 10), Interval::new(2, 3)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(2, 3), Interval::new(1, 10)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(1, 5), Interval::new(2, 7)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(2, 7), Interval::new(1, 5)),
            true
        );

        assert_eq!(
            Interval::overlaps(Interval::new(1, 5), Interval::new(0, 2)),
            true
        );
    }

    #[test]
    fn combine() {
        assert_eq!(
            Interval::combine(Interval::new(1, 5), Interval::new(0, 2)),
            Interval::new(0, 5)
        );

        assert_eq!(
            Interval::combine(Interval::new(1, 5), Interval::new(5, 10)),
            Interval::new(1, 10)
        );

        assert_eq!(
            Interval::combine(Interval::new(2, 7), Interval::new(1, 5)),
            Interval::new(1, 7)
        );
    }
}

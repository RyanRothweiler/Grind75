#![allow(dead_code, unused_variables)]

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    // QuickSelect

    fn origin_dist(first: &Vec<i32>) -> f64 {
        let second: Vec<i32> = vec![0, 0];

        return ((second[0] as f64 - first[0] as f64).powf(2.0)
            + (second[1] as f64 - first[1] as f64).powf(2.0))
        .sqrt();
    }

    fn quick_select(points: &mut Vec<Vec<i32>>, k: usize) -> Vec<Vec<i32>> {
        let pivot = points.pop().unwrap();
        let pivot_dist = origin_dist(&pivot);

        let mut low: Vec<Vec<i32>> = vec![];
        let mut high: Vec<Vec<i32>> = vec![];

        for p in points {
            let p_dist = origin_dist(p);

            println!("{:?}, {:?}", pivot_dist, p_dist);

            if p_dist <= pivot_dist {
                low.push(p.clone());
            } else if p_dist > pivot_dist {
                high.push(p.clone());
            }
        }

        low.push(pivot);

        if low.len() == k as usize {
            return low;
        } else if low.len() < k as usize {
            // low doesn't have enough elements. So pivot is in high somewhere.
            let mut assembled: Vec<Vec<i32>> = vec![];
            assembled.append(&mut low);
            assembled.append(&mut high);
            return quick_select(&mut assembled, k);
        } else {
            // low does have enouh elements. So pivot is in low somewhere.
            return quick_select(&mut low, k);
        }
    }

    if k as usize == points.len() {
        return points;
    }

    if k as usize > points.len() {
        panic!("Not enough points. Invalid input.");
    }

    return quick_select(&mut points.clone(), k as usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn dist() {
        assert_eq!(origin_dist(&vec![1, 3]), 2.82);
    }
    */

    #[test]
    fn first() {
        let points: Vec<Vec<i32>> = vec![vec![1, 3], vec![-2, -2]];
        let ret = k_closest(points, 1);
        assert_eq!(ret, vec![vec![-2, -2]]);
    }

    #[test]
    fn second() {
        let points: Vec<Vec<i32>> = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let ret = k_closest(points, 2);
        assert_eq!(ret, vec![vec![3, 3], vec![-2, 4]]);
    }

    #[test]
    fn third() {
        let points: Vec<Vec<i32>> = vec![vec![-2, 10], vec![-4, -8], vec![10, 7], vec![-4, -7]];
        let ret = k_closest(points, 3);
        assert_eq!(ret, vec![vec![3, 3], vec![-2, 4]]);
    }
}

fn main() {
    println!("Hello, world!");
}

fn index(i: i32, matrix: &Vec<Vec<i32>>) -> i32 {
    let width: i32 = matrix[0].len() as i32;

    let y: i32 = i / width;
    let x: i32 = i - (y * width);

    matrix[y as usize][x as usize]
}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut left: i32 = 0;
    let mut right: i32 = (matrix.len() * matrix[0].len()) as i32 - 1;

    while left <= right {
        let mid: i32 = left + (right - left) / 2;
        let mid_val = index(mid, &matrix);
        if mid_val == target {
            return true;
        } else if target < mid_val {
            right = mid - 1;
        } else if target > mid_val {
            left = mid + 1;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn indexing() {
        let map = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(index(0, &map), 1);
        assert_eq!(index(3, &map), 7);

        assert_eq!(index(4, &map), 10);

        assert_eq!(index(6, &map), 16);
        assert_eq!(index(7, &map), 20);

        assert_eq!(index(8, &map), 23);
        assert_eq!(index(11, &map), 60);
    }

    #[test]
    fn working() {
        let map = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];

        assert!(search_matrix(map.clone(), 1));
        assert!(search_matrix(map.clone(), 20));
        assert!(search_matrix(map.clone(), 60));
        assert!(search_matrix(map.clone(), 23));
        assert!(search_matrix(map.clone(), 7));
        assert!(search_matrix(map.clone(), 11));

        assert!(!search_matrix(map.clone(), 0));
        assert!(!search_matrix(map.clone(), 61));
    }
}

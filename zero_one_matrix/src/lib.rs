pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut result = mat.clone();
    let mut queue = std::collections::VecDeque::new();

    let max_value = m * n;

    for x in 0..m {
        for y in 0..n {
            if mat[x][y] == 0 {
                queue.push_back((x, y))
            } else {
                // this assume we don't have a grid who x * y is greater than i32::MAX
                result[x][y] = max_value as i32;
            }
        }
    }

    while let Some((row, col)) = queue.pop_front() {
        for (dr, dc) in &dirs {
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0
                && r < m as i32
                && c >= 0
                && c < n as i32
                && result[r as usize][c as usize] > result[row][col] + 1
            {
                queue.push_back((r as usize, c as usize));
                result[r as usize][c as usize] = result[row][col] + 1;
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {}
}

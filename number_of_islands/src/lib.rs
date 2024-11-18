pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    const ISLAND: char = '1';
    const WATER: char = '0';
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut islands_count: i32 = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == ISLAND {
                islands_count += 1;

                // run search to clear out entire island
                let mut stack: Vec<(usize, usize)> = vec![];
                stack.push((x, y));

                while !stack.is_empty() {
                    let pos = stack.pop().unwrap();
                    grid[pos.0][pos.1] = WATER;

                    for d in DIRS {
                        let mut new_pos: (i32, i32) = (0, 0);
                        new_pos.0 = (pos.0 as i32 + d.0).clamp(0, grid.len() as i32 - 1);
                        new_pos.1 = (pos.1 as i32 + d.1).clamp(0, grid[0].len() as i32 - 1);

                        let new_pos: (usize, usize) = (new_pos.0 as usize, new_pos.1 as usize);

                        if grid[new_pos.0][new_pos.1] == ISLAND {
                            stack.push((new_pos.0, new_pos.1));
                        }
                    }
                }
            }
        }
    }

    return islands_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}

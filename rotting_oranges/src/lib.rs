pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    if grid.len() == 0 {
        return 0;
    }

    // const EMPTY: i32 = 0;
    const FRESH: i32 = 1;
    const ROTTEN: i32 = 2;

    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut minute_count: i32 = 0;
    let mut fresh_count: i32 = 0;

    let mut rotten_frontier: VecDeque<(i32, i32)> = VecDeque::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let v = grid[x][y];

            if v == FRESH {
                fresh_count += 1;
            }
            if v == ROTTEN {
                rotten_frontier.push_back((x as i32, y as i32));
            }
        }
    }

    if fresh_count == 0 {
        return 0;
    }

    while !rotten_frontier.is_empty() {
        minute_count += 1;

        let process_count = rotten_frontier.len();

        for _i in 0..process_count {
            let frontier_pos = rotten_frontier.pop_front().unwrap();

            for d in DIRS {
                let mut adj_pos: (i32, i32) = (0, 0);
                adj_pos.0 = (frontier_pos.0 + d.0).clamp(0, grid.len() as i32 - 1);
                adj_pos.1 = (frontier_pos.1 + d.1).clamp(0, grid[0].len() as i32 - 1);

                if grid[adj_pos.0 as usize][adj_pos.1 as usize] == FRESH {
                    grid[adj_pos.0 as usize][adj_pos.1 as usize] = ROTTEN;
                    fresh_count -= 1;

                    rotten_frontier.push_back(adj_pos);
                }
            }
        }
    }

    if fresh_count == 0 {
        // Subtract one becuase the final edge of rotten oranges will be processed,
        // but then will add no other rottens
        return minute_count - 1;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
    }

    #[test]
    fn third() {
        assert_eq!(oranges_rotting(vec![vec![0, 2]]), 0);
    }
}

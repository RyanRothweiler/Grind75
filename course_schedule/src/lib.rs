pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut in_deg: Vec<i32> = vec![0; num_courses as usize];

    // setup adjacency list
    let mut adj_list: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
    for p in prerequisites {
        adj_list[p[1] as usize].push(p[0]);
        in_deg[p[0] as usize] += 1;
    }

    let mut q: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
    let mut visited: i32 = 0;

    // Add all nodes with no in-degree to the start
    for i in 0..in_deg.len() {
        if in_deg[i] == 0 {
            q.push_back(i as i32);
        }
    }

    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        visited += 1;

        for neigh in adj_list[n as usize].clone() {
            let nu = neigh as usize;

            in_deg[nu] -= 1;
            if in_deg[nu] == 0 {
                q.push_back(neigh);
            }
        }
    }

    return visited == num_courses;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(can_finish(2, vec![vec![0, 1]]), true);
    }
}

#![allow(unused_variables, dead_code, clippy::all, unused_mut)]
use std::collections::HashSet;

fn main() {
    can_finish(2, vec![vec![1, 0]]);
}

#[derive(Clone)]
struct CourseReq {
    reqs: Vec<usize>,
    known_good: bool,
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    fn dfs(course_reqs: &mut Vec<CourseReq>, visited: &mut HashSet<usize>, current: usize) -> bool {
        if course_reqs[current].known_good {
            return true;
        }

        if visited.contains(&current) {
            return false;
        }
        visited.insert(current);

        let reqs_list = course_reqs[current].reqs.clone();
        for req in reqs_list {
            if !dfs(course_reqs, visited, req) {
                return false;
            }
        }

        course_reqs[current].known_good = true;
        true
    }

    let mut course_reqs: Vec<CourseReq> = vec![
        CourseReq {
            reqs: vec![],
            known_good: false
        };
        num_courses as usize
    ];

    for p in prerequisites {
        course_reqs[p[0] as usize].reqs.push(p[1] as usize);
    }

    for n in 0..num_courses {
        if !dfs(&mut course_reqs, &mut HashSet::new(), n as usize) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(can_finish(2, vec![vec![1, 0]]), true);

        assert_eq!(
            can_finish(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            true
        );
        assert_eq!(can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(
            can_finish(5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]),
            true
        );
    }
}

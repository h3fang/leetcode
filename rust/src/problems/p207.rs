pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(mut num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut g = vec![vec![]; num_courses as usize];
        let mut deg = vec![0; num_courses as usize];
        for p in prerequisites {
            g[p[1] as usize].push(p[0] as usize);
            deg[p[0] as usize] += 1;
        }
        let mut q = VecDeque::new();
        for (i, &d) in deg.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }
        while let Some(i) = q.pop_front() {
            num_courses -= 1;
            for &j in &g[i] {
                deg[j] -= 1;
                if deg[j] == 0 {
                    q.push_back(j);
                }
            }
        }
        num_courses == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let prerequisites = [[1, 0]].iter().map(|p| p.to_vec()).collect();
        assert!(Solution::can_finish(2, prerequisites));
    }

    #[test]
    fn case2() {
        let prerequisites = [[1, 0], [0, 1]].iter().map(|p| p.to_vec()).collect();
        assert!(!Solution::can_finish(2, prerequisites));
    }
}

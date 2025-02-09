pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut in_deg = vec![0; n];
        favorite.iter().for_each(|&f| in_deg[f as usize] += 1);
        let mut visited = vec![false; n];
        let mut f = vec![1; n];
        let mut q = in_deg
            .iter()
            .enumerate()
            .filter(|e| *e.1 == 0)
            .map(|e| e.0)
            .collect::<VecDeque<_>>();
        while let Some(i) = q.pop_front() {
            visited[i] = true;
            let j = favorite[i] as usize;
            f[j] = f[j].max(f[i] + 1);
            in_deg[j] -= 1;
            if in_deg[j] == 0 {
                q.push_back(j);
            }
        }
        let (mut ring, mut ring2) = (0, 0);
        for i in 0..n {
            if visited[i] {
                continue;
            }
            let j = favorite[i] as usize;
            if i == favorite[j] as usize {
                ring2 += f[i] + f[j];
                visited[i] = true;
                visited[j] = true;
            } else {
                let (mut j, mut c) = (i, 0);
                loop {
                    c += 1;
                    j = favorite[j] as usize;
                    visited[j] = true;
                    if j == i {
                        break;
                    }
                }
                ring = ring.max(c);
            }
        }
        ring.max(ring2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_invitations(vec![2, 2, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::maximum_invitations(vec![1, 2, 0]));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::maximum_invitations(vec![3, 0, 1, 4, 1]));
    }
}

pub struct Solution;

use std::collections::VecDeque;

fn check(g: &[Vec<(i32, i32)>], deg: &[i32], m: i32, k: i64) -> bool {
    let n = deg.len();
    let mut deg = deg.to_vec();
    let mut f = vec![i64::MAX / 2; n];
    f[0] = 0;
    let mut q = VecDeque::with_capacity(n);
    q.push_back(0);

    while let Some(x) = q.pop_front() {
        if x == n as i32 - 1 {
            return f[x as usize] <= k;
        }

        for &(y, w) in &g[x as usize] {
            if w >= m {
                f[y as usize] = (f[x as usize] + w as i64).min(f[y as usize]);
            }
            deg[y as usize] -= 1;
            if deg[y as usize] == 0 {
                q.push_back(y);
            }
        }
    }

    false
}

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();

        let mut max = -1;
        let mut deg = vec![0; n];
        let mut g = vec![vec![]; n];
        for e in edges {
            let (x, y) = (e[0] as usize, e[1] as usize);
            if online[x] && online[y] {
                g[x].push((e[1], e[2]));
                deg[e[1] as usize] += 1;
                if x == 0 {
                    max = max.max(e[2]);
                }
            }
        }

        let mut q = VecDeque::with_capacity(n);
        for (i, &d) in deg.iter().enumerate().skip(1) {
            if d == 0 {
                q.push_back(i as i32);
            }
        }

        while let Some(x) = q.pop_front() {
            for &(y, _) in &g[x as usize] {
                deg[y as usize] -= 1;
                if deg[y as usize] == 0 && y > 0 {
                    q.push_back(y);
                }
            }
        }

        let (mut l, mut r) = (-1, max + 1);

        while l + 1 < r {
            let m = (l + 1).midpoint(r);
            if check(&g, &deg, m, k) {
                l = m;
            } else {
                r = m;
            }
        }

        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1, 5], [1, 3, 10], [0, 2, 3], [2, 3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let online = vec![true, true, true, true];
        let k = 10;
        assert_eq!(3, Solution::find_max_path_score(edges, online, k));
    }

    #[test]
    fn case2() {
        let edges = [
            [0, 1, 7],
            [1, 4, 5],
            [0, 2, 6],
            [2, 3, 6],
            [3, 4, 2],
            [2, 4, 6],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let online = vec![true, true, true, false, true];
        let k = 12;
        assert_eq!(6, Solution::find_max_path_score(edges, online, k));
    }
}

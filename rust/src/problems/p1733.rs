pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let lang = languages
            .into_iter()
            .map(|e| e.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let mut f = vec![0; n as usize + 1];
        let mut vis = vec![false; lang.len()];
        let mut total = 0;
        let mut add = |x: usize| {
            if vis[x] {
                return;
            }
            vis[x] = true;
            total += 1;
            for &y in &lang[x] {
                f[y as usize] += 1;
            }
        };
        for e in friendships {
            let (x, y) = (e[0] as usize - 1, e[1] as usize - 1);
            if lang[x].is_disjoint(&lang[y]) {
                add(x);
                add(y);
            }
        }
        total - f.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 2;
        let languages = vec![vec![1], vec![2], vec![1, 2]];
        let friendships = [[1, 2], [1, 3], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(1, Solution::minimum_teachings(n, languages, friendships));
    }

    #[test]
    fn case2() {
        let n = 3;
        let languages = vec![vec![2], vec![1, 3], vec![1, 2], vec![3]];
        let friendships = [[1, 4], [1, 2], [3, 4], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(2, Solution::minimum_teachings(n, languages, friendships));
    }
}

pub struct Solution;

fn dfs(g: &[Vec<i32>], x: i32, suspicious: &mut [bool]) {
    suspicious[x as usize] = true;
    for &y in &g[x as usize] {
        if !suspicious[y as usize] {
            dfs(g, y, suspicious);
        }
    }
}

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; n as usize];
        for i in &invocations {
            g[i[0] as usize].push(i[1]);
        }
        let mut suspicious = vec![false; n as usize];

        dfs(&g, k, &mut suspicious);

        for i in &invocations {
            if suspicious[i[1] as usize] && !suspicious[i[0] as usize] {
                return (0..n).collect();
            }
        }

        (0..n).filter(|&x| !suspicious[x as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let invocations = [[1, 2], [0, 1], [3, 2]]
            .iter()
            .map(|inv| inv.to_vec())
            .collect();
        let mut ans = Solution::remaining_methods(4, 1, invocations);
        ans.sort_unstable();
        assert_eq!(vec![0, 1, 2, 3], ans);
    }

    #[test]
    fn case2() {
        let invocations = [[1, 2], [0, 2], [0, 1], [3, 4]]
            .iter()
            .map(|inv| inv.to_vec())
            .collect();
        let mut ans = Solution::remaining_methods(5, 0, invocations);
        ans.sort_unstable();
        assert_eq!(vec![3, 4], ans);
    }

    #[test]
    fn case3() {
        let invocations = [[1, 2], [0, 1], [2, 0]]
            .iter()
            .map(|inv| inv.to_vec())
            .collect();
        let mut ans = Solution::remaining_methods(3, 2, invocations);
        ans.sort_unstable();
        assert_eq!(Vec::<i32>::new(), ans);
    }
}

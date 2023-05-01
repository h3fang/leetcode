pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        fn dfs(g: &[Vec<i32>], i: i32, inform_time: &[i32]) -> i32 {
            let mut max = 0;
            for &c in &g[i as usize] {
                max = max.max(dfs(g, c, inform_time));
            }
            max + inform_time[i as usize]
        }
        let mut g = vec![vec![]; n as usize];
        for (i, m) in manager.into_iter().enumerate() {
            if m >= 0 {
                g[m as usize].push(i as i32);
            }
        }
        dfs(&g, head_id, &inform_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::num_of_minutes(1, 0, vec![-1], vec![0]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0])
        );
    }
}

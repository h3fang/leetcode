pub struct Solution;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut g = vec![vec![0usize; 0]; n as usize];
        for r in &relations {
            g[r[1] as usize - 1].push(r[0] as usize - 1);
        }
        let mut t = vec![-1; n as usize + 1];

        fn dfs(i: usize, t: &mut Vec<i32>, g: &[Vec<usize>], time: &[i32]) -> i32 {
            if t[i] == -1 {
                let mut r = 0;
                for req in &g[i] {
                    r = r.max(dfs(*req, t, g, time));
                }
                t[i] = r + time[i];
            }
            t[i]
        }

        let mut result = 0;
        for i in 0..n as usize {
            result = result.max(dfs(i, &mut t, &g, &time));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let relations = vec![vec![1, 3], vec![2, 3]];
        let time = vec![3, 2, 5];
        assert_eq!(8, Solution::minimum_time(n, relations, time));
    }

    #[test]
    fn case2() {
        let n = 5;
        let relations = vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]];
        let time = vec![1, 2, 3, 4, 5];
        assert_eq!(12, Solution::minimum_time(n, relations, time));
    }
}

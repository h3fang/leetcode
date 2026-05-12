pub struct Solution;

impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|t| t[1] - t[0]);

        let mut ans = 0;

        for t in tasks {
            ans = (ans + t[0]).max(t[1]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tasks = [[1, 2], [2, 4], [4, 8]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(8, Solution::minimum_effort(tasks));
    }

    #[test]
    fn case2() {
        let tasks = [[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(32, Solution::minimum_effort(tasks));
    }

    #[test]
    fn case3() {
        let tasks = [[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(27, Solution::minimum_effort(tasks));
    }
}

pub struct Solution;

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut d: Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
        d.sort_unstable();
        worker.sort_unstable();
        let mut result = 0;
        let (mut i, mut max) = (0, 0);
        for w in worker {
            while i < d.len() && d[i].0 <= w {
                max = max.max(d[i].1);
                i += 1;
            }
            result += max;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            100,
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            )
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25])
        );
    }
}

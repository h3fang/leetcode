pub struct Solution;

impl Solution {
    pub fn min_increments(n: i32, mut cost: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in (1..n as usize - 1).step_by(2).rev() {
            result += (cost[i] - cost[i + 1]).abs();
            cost[i / 2] += cost[i].max(cost[i + 1]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_increments(3, vec![5, 3, 3]));
    }
}

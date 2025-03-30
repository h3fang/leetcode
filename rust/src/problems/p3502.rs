pub struct Solution;

impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; cost.len()];
        let mut min = i32::MAX;
        for (i, c) in cost.into_iter().enumerate() {
            min = min.min(c);
            ans[i] = min;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![5, 3, 3, 1, 1, 1],
            Solution::min_costs(vec![5, 3, 4, 1, 3, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 1, 1, 1, 1],
            Solution::min_costs(vec![1, 2, 4, 6, 7])
        );
    }
}

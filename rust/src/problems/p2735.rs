pub struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut f = nums.clone();
        let mut result = nums.iter().map(|e| *e as i64).sum::<i64>();
        for k in 1..n {
            for i in 0..n {
                f[i] = f[i].min(nums[(i + k) % n]);
            }
            result = result.min(k as i64 * x as i64 + f.iter().map(|e| *e as i64).sum::<i64>());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::min_cost(vec![20, 1, 15], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::min_cost(vec![1, 2, 3], 4));
    }
}

pub struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut f = [0, i32::MIN, i32::MIN];
        for e in nums.into_iter() {
            let mut g = f;
            for i in 0..3 {
                let j = (i + e as usize) % 3;
                g[j] = f[j].max(f[i] + e);
            }
            f = g;
        }
        f[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(18, Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_sum_div_three(vec![1]));
    }

    #[test]
    fn case3() {
        assert_eq!(12, Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]));
    }
}

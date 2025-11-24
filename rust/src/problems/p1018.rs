pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut ans = Vec::with_capacity(nums.len());
        let mut x = 0;
        for b in nums {
            x = ((x << 1) | b) % 5;
            ans.push(x == 0);
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
            vec![true, false, false],
            Solution::prefixes_div_by5(vec![0, 1, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![false, false, false],
            Solution::prefixes_div_by5(vec![1, 1, 1])
        );
    }
}

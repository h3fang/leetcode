pub struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut freq = [0; 101];
        for n in nums {
            freq[n as usize] += 1;
        }
        freq.into_iter()
            .map(|f| if f > 1 { f * (f - 1) / 2 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::num_identical_pairs(vec![1, 1, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::num_identical_pairs(vec![1, 2, 3]));
    }
}

pub struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for (i, &x) in nums.iter().enumerate() {
            for (j, &y) in nums.iter().enumerate().skip(i + 1) {
                if x == y && (i * j) % (k as usize) == 0 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_pairs(vec![1, 2, 3, 4], 1));
    }
}

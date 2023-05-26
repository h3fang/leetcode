pub struct Solution;

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let (mut odd_p, mut odd_s, mut even_p, mut even_s) = (0, 0, 0, 0);
        for (i, n) in nums.iter().enumerate() {
            if i & 1 == 1 {
                odd_s += n;
            } else {
                even_s += n;
            }
        }
        for (i, n) in nums.iter().enumerate() {
            if i & 1 == 1 {
                odd_s -= n;
            } else {
                even_s -= n;
            }
            if odd_p + even_s == odd_s + even_p {
                result += 1;
            }
            if i & 1 == 1 {
                odd_p += n;
            } else {
                even_p += n;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::ways_to_make_fair(vec![2, 1, 6, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::ways_to_make_fair(vec![1, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::ways_to_make_fair(vec![1, 2, 3]));
    }
}

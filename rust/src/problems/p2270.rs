pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut suffix_sim = vec![0i64; nums.len() + 1];
        for i in (0..nums.len()).rev() {
            suffix_sim[i] = suffix_sim[i + 1] + nums[i] as i64;
        }
        let mut sum = 0;
        let mut result = 0;
        for i in 0..nums.len() - 1 {
            sum += nums[i] as i64;
            if sum >= suffix_sim[i + 1] {
                result += 1;
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
        assert_eq!(2, Solution::ways_to_split_array(vec![10, 4, -8, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::ways_to_split_array(vec![6, -1, 9]));
    }
}

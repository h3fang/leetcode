pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut a = 0i32;
        let mut b = nums.len() as i32 - 1;
        while a <= b {
            let i = (b + a) / 2;
            match nums[i as usize].cmp(&target) {
                std::cmp::Ordering::Less => a = i + 1,
                std::cmp::Ordering::Equal => return i,
                std::cmp::Ordering::Greater => b = i - 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 8));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::search(vec![5], -5));
    }
}

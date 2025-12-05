pub struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 0 {
            nums.len() as i32 - 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_partitions(vec![10, 10, 3, 7, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_partitions(vec![1, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::count_partitions(vec![2, 4, 6, 8]));
    }
}

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for num in nums {
            ones ^= num & !twos;
            twos ^= num & !ones;
        }
        ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    }
}

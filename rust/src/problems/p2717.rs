pub struct Solution;

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let i = nums.iter().position(|&x| x == 1).unwrap();
        let j = nums.iter().position(|&x| x == n as i32).unwrap();
        let steps = (i + n - 1 - j) as i32;
        if i < j {
            steps
        } else {
            steps - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::semi_ordered_permutation(vec![2, 1, 4, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::semi_ordered_permutation(vec![2, 4, 1, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]));
    }
}

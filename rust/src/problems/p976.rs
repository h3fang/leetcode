pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        for w in nums.windows(3).rev() {
            if w[0] + w[1] > w[2] {
                return w.iter().sum();
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::largest_perimeter(vec![2, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::largest_perimeter(vec![1, 1, 2]));
    }
}

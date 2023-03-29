pub struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable();
        let mut sum = 0;
        let mut result = 0;
        for s in satisfaction.into_iter().rev() {
            sum += s;
            if sum < 0 {
                break;
            }
            result += sum;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(14, Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]));
    }

    #[test]
    fn case2() {
        assert_eq!(20, Solution::max_satisfaction(vec![4, 3, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::max_satisfaction(vec![-1, -4, -5]));
    }
}

pub struct Solution;

impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let (mut max, mut sum) = (0, 0);
        for x in milestones {
            max = max.max(x);
            sum += x as i64;
        }
        let max = max as i64;
        if max > sum - max + 1 {
            2 * (sum - max) + 1
        } else {
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::number_of_weeks(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::number_of_weeks(vec![5, 2, 1]));
    }
}

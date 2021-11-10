pub struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut healthy = -1;
        let mut result = 0;
        for t in time_series {
            if t >= healthy {
                result += duration;
            } else {
                result += t + duration - healthy;
            }
            healthy = t + duration;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::find_poisoned_duration(vec![1, 4], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::find_poisoned_duration(vec![1, 2], 2));
    }
}

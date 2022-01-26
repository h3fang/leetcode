pub struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut increasing = false;
        let mut decreasing = false;
        for w in arr.windows(2) {
            if w[1] > w[0] && !decreasing {
                increasing = true;
            } else if w[1] < w[0] && increasing {
                decreasing = true;
            } else {
                return false;
            }
        }
        increasing && decreasing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(false, Solution::valid_mountain_array(vec![2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::valid_mountain_array(vec![3, 5, 5]));
    }

    #[test]
    fn case3() {
        assert_eq!(true, Solution::valid_mountain_array(vec![0, 3, 2, 1]));
    }
}

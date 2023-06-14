pub struct Solution;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut result = 0;
        for (i, f) in flips.into_iter().enumerate() {
            max = max.max(f);
            if i as i32 + 1 == max {
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
        assert_eq!(2, Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::num_times_all_blue(vec![4, 1, 2, 3]));
    }
}

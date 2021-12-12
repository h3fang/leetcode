pub struct Solution;

impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let n = security.len();
        let mut result = vec![];
        let mut inc = vec![0; n];
        let mut dec = 0;

        for i in (0..n - 1).rev() {
            if security[i] <= security[i + 1] {
                inc[i] = inc[i + 1] + 1;
            }
        }

        for i in 0..n {
            if i > 0 {
                if security[i] <= security[i - 1] {
                    dec += 1;
                } else {
                    dec = 0;
                }
            }
            if dec >= time && inc[i] >= time {
                result.push(i as i32);
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
        assert_eq!(
            vec![2, 3],
            Solution::good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 1, 2, 3, 4],
            Solution::good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![0; 0],
            Solution::good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2)
        );
    }
}

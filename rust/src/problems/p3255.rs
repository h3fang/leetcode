pub struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }
        let mut f = 1;
        let mut result = Vec::with_capacity(nums.len() + 1 - k as usize);
        for (i, w) in nums.windows(2).enumerate() {
            if w[1] == w[0] + 1 {
                f += 1;
            } else {
                f = 1;
            }
            if i + 2 >= k as usize {
                result.push(if f >= k { w[1] } else { -1 });
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
            vec![3, 4, -1, -1, -1],
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1, -1],
            Solution::results_array(vec![2, 2, 2, 2, 2], 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![-1, 3, -1, 3, -1],
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2)
        );
    }
}


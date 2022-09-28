pub struct Solution;

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut inc = vec![1; n];
        for i in (k as usize..n - 1).rev() {
            if nums[i] <= nums[i + 1] {
                inc[i] = inc[i + 1] + 1;
            }
        }
        let mut result = vec![];
        let mut dec = 1;
        for i in 1..n - k as usize {
            if dec >= k && inc[i + 1] >= k {
                result.push(i as i32);
            }
            if nums[i] <= nums[i - 1] {
                dec += 1;
            } else {
                dec = 1;
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
            Solution::good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0; 0], Solution::good_indices(vec![2, 1, 1, 2], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![4, 5],
            Solution::good_indices(
                vec![878724, 201541, 179099, 98437, 35765, 327555, 475851, 598885, 849470, 943442],
                4
            )
        );
    }
}

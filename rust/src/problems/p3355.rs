pub struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut diff = vec![0; nums.len() + 1];
        for q in queries {
            diff[q[0] as usize] += 1;
            diff[q[1] as usize + 1] -= 1;
        }
        let mut sum = 0;
        for (x, d) in nums.into_iter().zip(diff) {
            sum += d;
            if x > sum {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[0, 2]].iter().map(|q| q.to_vec()).collect();
        assert!(Solution::is_zero_array(vec![1, 0, 1], queries));
    }

    #[test]
    fn case2() {
        let queries = [[1, 3], [0, 2]].iter().map(|q| q.to_vec()).collect();
        assert!(!Solution::is_zero_array(vec![4, 3, 2, 1], queries));
    }
}

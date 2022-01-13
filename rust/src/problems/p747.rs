pub struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut twice = 0;
        let mut idx = -1;
        let mut max = 0;
        for (i, n) in nums.into_iter().enumerate() {
            twice = twice.max(2 * n);
            if n > max {
                twice = 2 * max;
                max = n;
                idx = i as i32;
            } else if 2 * n > twice {
                twice = 2 * n;
            }
        }
        if max >= twice {
            idx
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::dominant_index(vec![3, 6, 1, 0]));
    }
}

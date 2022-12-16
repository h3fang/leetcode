pub struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len() as i32;
        let mut left = -1;
        let mut right = 0;
        let mut result = 0;
        for (i, &s) in seats.iter().enumerate() {
            if s == 1 {
                left = i as i32;
            } else {
                while (right < n && seats[right as usize] == 0) || right < i as i32 {
                    right += 1;
                }
                let left = if left == -1 { n } else { i as i32 - left };
                let right = if right == n { n } else { right - i as i32 };
                result = result.max(left.min(right));
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
        assert_eq!(2, Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::max_dist_to_closest(vec![1, 0, 0, 0]));
    }
}

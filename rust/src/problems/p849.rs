pub struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut l = 0;
        while l < n && seats[l] == 0 {
            l += 1;
        }
        let mut result = l as i32;
        while l < n {
            let mut r = l + 1;
            while r < n && seats[r] == 0 {
                r += 1;
            }
            let d = if r == n { n - l - 1 } else { (r - l) / 2 };
            result = result.max(d as i32);
            l = r;
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

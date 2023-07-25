pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len() - 1;
        while left < right {
            let m = (right - left) / 2 + left;
            if arr[m] < arr[m + 1] {
                left = m + 1;
            } else {
                right = m;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 1, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]));
    }
}

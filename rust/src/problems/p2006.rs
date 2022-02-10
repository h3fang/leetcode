pub struct Solution;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut c = [0; 101];
        let mut result = 0;
        for n in nums {
            if k + n > 0 && k + n <= 100 {
                result += c[(k + n) as usize];
            }

            if n - k > 0 && n - k <= 100 {
                result += c[(n - k) as usize];
            }
            c[n as usize] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_k_difference(vec![1, 2, 2, 1], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_k_difference(vec![1, 3], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2));
    }
}

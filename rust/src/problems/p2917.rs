pub struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = [0; 32];
        for n in nums {
            for (i, b) in bits.iter_mut().enumerate() {
                if (1 << i) & n > 0 {
                    *b += 1;
                }
            }
        }
        bits.into_iter()
            .rev()
            .fold(0, |x, b| (x << 1) + i32::from(b >= k))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::find_k_or(vec![2, 12, 1, 11, 4, 5], 6));
    }

    #[test]
    fn case3() {
        assert_eq!(15, Solution::find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1));
    }
}

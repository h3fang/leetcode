pub struct Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut s = [false; 2001];
        let mut k = i32::MIN;
        for n in nums {
            s[(n + 1000) as usize] = true;
            if s[(1000 - n) as usize] {
                k = k.max(n.abs());
            }
        }
        if k == i32::MIN {
            -1
        } else {
            k
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::find_max_k(vec![-1, 2, -3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]));
    }
}

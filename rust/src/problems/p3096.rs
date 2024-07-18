pub struct Solution;

impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let n = possible.len();
        let sum: i32 = possible.iter().map(|&x| if x == 1 { 1 } else { -1 }).sum();
        let mut pre = 0;
        for (i, x) in possible.into_iter().enumerate().take(n - 1) {
            pre += if x == 1 { 1 } else { -1 };
            if pre > sum - pre {
                return i as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_levels(vec![1, 0, 1, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimum_levels(vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::minimum_levels(vec![0, 0]));
    }
}

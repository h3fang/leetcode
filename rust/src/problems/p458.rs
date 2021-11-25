pub struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let t = minutes_to_test / minutes_to_die + 1;
        let mut k = 0;
        let mut c = 1;
        while c < buckets {
            c *= t;
            k += 1;
        }
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::poor_pigs(1000, 15, 60));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::poor_pigs(4, 15, 15));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::poor_pigs(4, 15, 30));
    }
}

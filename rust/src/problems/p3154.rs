pub struct Solution;

impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        fn n_choose_k(n: i32, k: i32) -> i32 {
            let mut r = 1i64;
            for i in ((n - k + 1)..=n).rev() {
                r *= i as i64;
                r /= (n - i + 1) as i64;
            }
            r as i32
        }
        let (mut result, mut n, mut pow) = (0, 0, 1);
        while pow - n - 1 <= k {
            if k <= pow {
                result += n_choose_k(n + 1, pow - k);
            }
            n += 1;
            pow *= 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::ways_to_reach_stair(0));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::ways_to_reach_stair(1));
    }
}

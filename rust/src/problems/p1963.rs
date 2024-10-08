pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        // let s = s.as_bytes();
        let mut c = 0;
        let mut min = 0;
        for b in s.bytes() {
            match b {
                b'[' => c += 1,
                b']' => {
                    c -= 1;
                    if c < 0 {
                        min = min.min(c);
                    }
                }
                _ => unreachable!(),
            }
        }
        (1 - min) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_swaps("][][".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_swaps("]]][[[".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_swaps("[]".to_string()));
    }
}

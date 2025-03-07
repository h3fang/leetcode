pub struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        fn valid(bloom_day: &[i32], mut m: i32, k: i32, day: i32) -> bool {
            let mut c = 0;
            for &x in bloom_day {
                if x <= day {
                    c += 1;
                    if c == k {
                        m -= 1;
                        if m == 0 {
                            return true;
                        }
                        c = 0;
                    }
                } else {
                    c = 0;
                }
            }
            false
        }
        if m * k > bloom_day.len() as i32 {
            return -1;
        }
        let (mut l, mut r) = (1, *bloom_day.iter().max().unwrap());
        let mut result = -1;
        while l <= r {
            let day = (r - l) / 2 + l;
            if valid(&bloom_day, m, k, day) {
                result = day;
                r = day - 1;
            } else {
                l = day + 1;
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
        assert_eq!(3, Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2));
    }

    #[test]
    fn case3() {
        assert_eq!(12, Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3));
    }
}

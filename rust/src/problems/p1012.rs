pub struct Solution;

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        fn perm(mut n: i32, k: i32) -> i32 {
            (0..k).fold(1, |mut r, _| {
                r *= n;
                n -= 1;
                r
            })
        }
        fn f(mask: u16, s: &str, i: i32, same: bool) -> i32 {
            if i == s.len() as i32 {
                return 1;
            }
            let t = if same {
                s.as_bytes()[i as usize] - b'0'
            } else {
                9
            };
            let mut result = 0;
            let c = mask.count_ones() as i32 + 1;
            for k in 0..=t {
                if mask & (1 << k) > 0 {
                    continue;
                }
                if same && k == t {
                    result += f(mask | (1 << t), s, i + 1, true);
                } else if mask == 0 && k == 0 {
                    result += f(0, s, i + 1, false);
                } else {
                    result += perm(10 - c, s.len() as i32 - 1 - i);
                }
            }
            result
        }
        let s = n.to_string();
        n + 1 - f(0, &s, 0, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::num_dup_digits_at_most_n(20));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::num_dup_digits_at_most_n(100));
    }

    #[test]
    fn case3() {
        assert_eq!(262, Solution::num_dup_digits_at_most_n(1000));
    }
}

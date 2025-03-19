pub struct Solution;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let s = croak_of_frogs.as_bytes();
        if s.len() % 5 != 0 {
            return -1;
        }
        fn pos(b: u8) -> usize {
            match b {
                b'c' => 0,
                b'r' => 1,
                b'o' => 2,
                b'a' => 3,
                b'k' => 4,
                _ => unreachable!(),
            }
        }
        let (mut result, mut frogs) = (0, 0);
        let mut cnt = [0; 4];
        for &b in s {
            let i = pos(b);
            if i == 0 {
                frogs += 1;
                cnt[i] += 1;
                result = result.max(frogs);
            } else {
                if cnt[i - 1] == 0 {
                    return -1;
                }
                cnt[i - 1] -= 1;
                if i == 4 {
                    frogs -= 1;
                } else {
                    cnt[i] += 1;
                }
            }
        }
        if frogs > 0 { -1 } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_number_of_frogs("croakcroak".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_number_of_frogs("crcoakroak".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_number_of_frogs("croakcrook".to_string()));
    }
}

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        if n1 == 0 {
            return 0;
        }
        let (mut i, mut i2) = (0, 0);
        let mut m: HashMap<usize, (i32, i32)> = HashMap::new();
        let (mut pre, mut seg) = ((0, 0), (0, 0));
        for i1 in 1..=n1 {
            for &b in s1 {
                if b == s2[i] {
                    i += 1;
                    if i == s2.len() {
                        i2 += 1;
                        i = 0;
                    }
                }
            }
            if i1 == n1 {
                return i2 / n2;
            }
            match m.get(&i) {
                Some(&(a, b)) => {
                    pre = (a, b);
                    seg = (i1 - a, i2 - b);
                    break;
                }
                None => {
                    m.insert(i, (i1, i2));
                }
            }
        }
        let mut result = pre.1 + (n1 - pre.0) / seg.0 * seg.1;
        for _ in 0..(n1 - pre.0) % seg.0 {
            for &b in s1 {
                if b == s2[i] {
                    i += 1;
                    if i == s2.len() {
                        result += 1;
                        i = 0;
                    }
                }
            }
        }
        result / n2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::get_max_repetitions("acb".to_string(), 1, "acb".to_string(), 1)
        );
    }
}

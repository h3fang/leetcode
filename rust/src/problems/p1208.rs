pub struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let (mut result, mut cost, mut l) = (0, 0, 0);
        let (s, t) = (s.as_bytes(), t.as_bytes());
        for (r, &x) in s.iter().enumerate() {
            cost += x.abs_diff(t[r]) as i32;
            while cost > max_cost {
                cost -= s[l].abs_diff(t[l]) as i32;
                l += 1;
            }
            if r >= l {
                result = result.max(r + 1 - l);
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            1,
            Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0)
        );
    }
}

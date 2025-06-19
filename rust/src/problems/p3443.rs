pub struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let (mut x, mut y) = (0i32, 0i32);
        let mut ans = 0;
        for (i, b) in s.bytes().enumerate() {
            match b {
                b'N' => y += 1,
                b'E' => x += 1,
                b'S' => y -= 1,
                b'W' => x -= 1,
                _ => unreachable!(),
            }
            ans = ans.max((x.abs() + y.abs() + 2 * k).min(i as i32 + 1));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_distance("NWSE".to_string(), 1));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::max_distance("NSWWEW".to_string(), 3));
    }
}

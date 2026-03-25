pub struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut m = [0; 1024];
        m[0] = 1;
        let (mut pre, mut result) = (0, 0);
        for &b in word.as_bytes() {
            pre ^= 1 << (b - b'a');
            result += m[pre as usize];
            for i in 0..10 {
                let x = pre ^ (1 << i);
                result += m[x as usize];
            }
            m[pre as usize] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::wonderful_substrings("aba".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::wonderful_substrings("aabb".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::wonderful_substrings("he".to_string()));
    }
}

pub struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut c = [0; 26];
        for b in text.as_bytes() {
            c[(b - b'a') as usize] += 1;
        }
        println!("{:?}", c);
        let mut n = c[1];
        n = n.min(c[0]);
        n = n.min(c[(b'l' - b'a') as usize] / 2);
        n = n.min(c[(b'o' - b'a') as usize] / 2);
        n = n.min(c[(b'n' - b'a') as usize]);
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::max_number_of_balloons("nlaebolko".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::max_number_of_balloons("loonbalxballpoon".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::max_number_of_balloons("leetcode".to_string()));
    }
}

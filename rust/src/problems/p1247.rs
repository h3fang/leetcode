pub struct Solution;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let (mut xy, mut yx) = (0, 0);
        for (a, b) in s1.chars().zip(s2.chars()) {
            match (a, b) {
                ('x', 'y') => xy += 1,
                ('y', 'x') => yx += 1,
                _ => {}
            }
        }
        if (xy + yx) % 2 == 1 {
            return -1;
        }
        xy / 2 + yx / 2 + xy % 2 + yx % 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::minimum_swap("xx".to_string(), "yy".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::minimum_swap("xy".to_string(), "yx".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            -1,
            Solution::minimum_swap("xx".to_string(), "yx".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            4,
            Solution::minimum_swap("xxyyxyxyxx".to_string(), "xyyxyxxxyx".to_string())
        );
    }
}

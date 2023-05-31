pub struct Solution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut s = vec![];
        for x in arr {
            while !s.is_empty() && *s.last().unwrap() <= x {
                let y = s.pop().unwrap();
                if s.is_empty() || *s.last().unwrap() > x {
                    result += y * x;
                } else {
                    result += *s.last().unwrap() * y;
                }
            }
            s.push(x);
        }
        while s.len() >= 2 {
            let x = s.pop().unwrap();
            result += *s.last().unwrap() * x;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(32, Solution::mct_from_leaf_values(vec![6, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(44, Solution::mct_from_leaf_values(vec![4, 11]));
    }
}

pub struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut s = Vec::with_capacity(arr.len());
        for a in arr {
            if s.is_empty() || a >= *s.last().unwrap() {
                s.push(a);
            } else {
                let last = s.pop().unwrap();
                while !s.is_empty() && *s.last().unwrap() > a {
                    s.pop().unwrap();
                }
                s.push(last);
            }
        }
        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]));
    }
}

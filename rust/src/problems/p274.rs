pub struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        let mut h = 0;
        for x in citations.into_iter().rev() {
            if x > h {
                h += 1;
            } else {
                break;
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::h_index(vec![3, 0, 6, 1, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::h_index(vec![1, 3, 1]));
    }
}

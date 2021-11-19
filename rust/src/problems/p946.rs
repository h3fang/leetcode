pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let m = pushed.len();
        let mut i = 0;
        let mut s = Vec::with_capacity(m);
        for n in popped {
            while s.is_empty() || *s.last().unwrap() != n {
                if i == m {
                    return false;
                }
                s.push(pushed[i]);
                i += 1;
            }
            s.pop();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert_eq!(true, Solution::validate_stack_sequences(pushed, popped));
    }

    #[test]
    fn case2() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        assert_eq!(false, Solution::validate_stack_sequences(pushed, popped));
    }
}

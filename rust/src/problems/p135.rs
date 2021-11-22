pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut pre = 1;
        let mut inc = 1;
        let mut dec = 0;
        for (i, &r) in ratings.iter().enumerate().skip(1) {
            match r.cmp(&ratings[i - 1]) {
                std::cmp::Ordering::Less => {
                    dec += 1;
                    if dec == inc {
                        dec += 1;
                    }
                    pre = 1;
                    result += dec;
                }
                std::cmp::Ordering::Equal => {
                    dec = 0;
                    inc = 1;
                    pre = 1;
                    result += 1;
                }
                std::cmp::Ordering::Greater => {
                    pre += 1;
                    dec = 0;
                    inc = pre;
                    result += pre;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let ratings = vec![1, 0, 2];
        assert_eq!(5, Solution::candy(ratings));
    }

    #[test]
    fn case2() {
        let ratings = vec![1, 2, 2];
        assert_eq!(4, Solution::candy(ratings));
    }
}

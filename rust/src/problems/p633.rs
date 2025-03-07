pub struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let (mut l, mut r) = (0, ((c + 1) as f64).sqrt() as i64);
        while l <= r {
            let s = l * l + r * r;
            match s.cmp(&c) {
                std::cmp::Ordering::Less => l += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => r -= 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::judge_square_sum(5));
    }

    #[test]
    fn case2() {
        assert!(!Solution::judge_square_sum(3));
    }
}

pub struct Solution;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let n = hours.len();
        let mut ps = vec![0; n + 1];
        let mut s = vec![0];
        for i in 1..=n {
            ps[i] = ps[i - 1] + if hours[i - 1] > 8 { 1 } else { -1 };
            if ps[*s.last().unwrap()] > ps[i] {
                s.push(i);
            }
        }
        let mut result = 0;
        for r in (1..=n).rev() {
            while !s.is_empty() && ps[*s.last().unwrap()] < ps[r] {
                result = result.max(r as i32 - *s.last().unwrap() as i32);
                s.pop();
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
        assert_eq!(3, Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::longest_wpi(vec![6, 6, 6]));
    }
}

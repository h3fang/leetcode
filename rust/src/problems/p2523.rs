pub struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let (left, right) = (left as usize, right as usize);
        let mut f = vec![true; right + 1];
        f[0] = false;
        f[1] = false;
        let ub = (right as f64).sqrt().round() as usize;
        for i in 2..=ub {
            if !f[i] {
                continue;
            }
            for j in (2 * i..=right).step_by(i) {
                f[j] = false;
            }
        }
        let mut ans = vec![-1, -1];
        let (mut prev, mut min) = (0, right - left + 1);
        for (i, &e) in f.iter().enumerate().take(right + 1).skip(left) {
            if e {
                if prev > 0 && i - prev < min {
                    min = i - prev;
                    ans[0] = prev as i32;
                    ans[1] = i as i32;
                }
                prev = i;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![11, 13], Solution::closest_primes(10, 19));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![-1, -1], Solution::closest_primes(4, 6));
    }
}

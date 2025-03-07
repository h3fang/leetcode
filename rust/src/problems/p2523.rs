pub struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let (left, right) = (left as usize, right as usize);
        let mut primes = Vec::with_capacity(1024);
        let mut f = vec![true; right + 1];
        for x in 2..=right {
            if f[x] {
                primes.push(x as i32);
            }
            for &y in &primes {
                if y as usize * x > right {
                    break;
                }
                f[y as usize * x] = false;
                if y as usize % x == 0 {
                    break;
                }
            }
        }
        let mut ans = vec![-1, -1];
        let mut min = (right - left + 1) as i32;
        let i = primes.partition_point(|&x| x < left as i32);
        for w in primes[i..].windows(2) {
            if w[1] > right as i32 {
                break;
            }
            if w[1] - w[0] < min {
                min = w[1] - w[0];
                ans[0] = w[0];
                ans[1] = w[1];
                if min == 2 {
                    break;
                }
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

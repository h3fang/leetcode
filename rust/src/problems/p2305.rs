pub struct Solution;

impl Solution {
    pub fn distribute_cookies(mut cookies: Vec<i32>, k: i32) -> i32 {
        fn distribute(cookies: &[i32], i: usize, dist: &mut [i32], k: i32, max: i32) -> bool {
            if i == cookies.len() {
                return true;
            }
            let c = cookies[i];
            for j in 0..dist.len() {
                if dist[j] + c <= max {
                    dist[j] += c;
                    if distribute(cookies, i + 1, dist, k, max) {
                        return true;
                    }
                    dist[j] -= c;
                }
            }
            false
        }
        cookies.sort_unstable_by(|a, b| b.cmp(a));
        let mut left = 1;
        let mut right = cookies.iter().sum::<i32>();
        let mut result = right;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mut dist = vec![0; k as usize];
            if distribute(&cookies, 0, &mut dist, k, mid) {
                result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
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
        let cookies = vec![8, 15, 10, 20, 8];
        let k = 2;
        assert_eq!(31, Solution::distribute_cookies(cookies, k));
    }

    #[test]
    fn case2() {
        let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
        let k = 3;
        assert_eq!(7, Solution::distribute_cookies(cookies, k));
    }
}

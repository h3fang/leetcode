pub struct Solution;

fn check(power: &[i64], mut k: usize, r: usize, low: i64) -> bool {
    let n = power.len();
    let mut diff = vec![0; n + 1];
    let mut sum = 0;
    for (i, p) in power.iter().enumerate() {
        sum += diff[i];
        let m = low - (p + sum);
        if m < 0 {
            continue;
        }
        if k < m as usize {
            return false;
        }
        k -= m as usize;
        sum += m;
        diff[n.min(i + 2 * r + 1)] -= m;
    }
    true
}

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let (n, r, k) = (stations.len(), r as usize, k as usize);
        let mut pre_sum = Vec::with_capacity(n + 1);
        pre_sum.push(0);
        for &x in &stations {
            pre_sum.push(pre_sum.last().unwrap() + x as i64);
        }

        let mut power = vec![0i64; n];
        let mut min = i64::MAX;
        for (i, x) in power.iter_mut().enumerate() {
            *x = pre_sum[n.min(i + r + 1)] - pre_sum[i.saturating_sub(r)];
            min = min.min(*x);
        }

        let (mut left, mut right) = (min + (k / n) as i64, min + k as i64 + 1);
        while left + 1 < right {
            let m = left + (right - left) / 2;
            if check(&power, k, r, m) {
                left = m;
            } else {
                right = m;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_power(vec![4, 4, 4, 4], 0, 3));
    }
}

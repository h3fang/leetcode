pub struct Solution;

impl Solution {
    pub fn min_time(_s: String, order: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (order.len(), k as i64);
        let mut count = (n * (n + 1) / 2) as i64;
        if k > count {
            return -1;
        }

        let mut prev: Vec<i64> = (-1..n as i64).collect();
        let mut next: Vec<i64> = (1..=n as i64).collect();

        for (t, i) in order.into_iter().enumerate().rev() {
            let l = prev[i as usize];
            let r = next[i as usize];
            count -= (i as i64 - l) * (r - i as i64);
            if count < k {
                return t as i32;
            }
            if l >= 0 {
                next[l as usize] = r;
            }
            prev[r as usize] = l;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abc".to_string();
        let order = vec![1, 0, 2];
        let k = 2;
        assert_eq!(0, Solution::min_time(s, order, k));
    }

    #[test]
    fn case2() {
        let s = "cat".to_string();
        let order = vec![0, 2, 1];
        let k = 6;
        assert_eq!(2, Solution::min_time(s, order, k));
    }

    #[test]
    fn case3() {
        let s = "xy".to_string();
        let order = vec![0, 1];
        let k = 4;
        assert_eq!(-1, Solution::min_time(s, order, k));
    }
}

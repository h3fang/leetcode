pub struct Solution;

fn count(mut order: Vec<i32>, all: usize, k: usize, n: usize) -> bool {
    order.sort_unstable();
    let (mut invalid, mut prev) = (0, -1);
    for i in order.into_iter().chain([n as i32]) {
        let n = (i - prev - 1) as usize;
        invalid += n * (n + 1) / 2;
        if all - invalid < k {
            return false;
        }
        prev = i;
    }
    all - invalid >= k
}

impl Solution {
    pub fn min_time(_s: String, order: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (order.len(), k as usize);
        let all = n * (n + 1) / 2;
        if k > all {
            return -1;
        }

        let (mut l, mut r, mut ans) = (0, n as i32 - 1, -1);
        while l <= r {
            let m = (l + r) / 2;
            if count(order[..=m as usize].to_vec(), all, k, n) {
                ans = m;
                r = m - 1;
            } else {
                l = m + 1;
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

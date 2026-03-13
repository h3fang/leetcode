pub struct Solution;

fn check(workers: &[i32], mut h: i64, min: i64) -> bool {
    for &w in workers {
        let dh = ((8 * min / w as i64 + 1).isqrt() - 1) / 2;
        h -= dh;
        if h <= 0 {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let h = mountain_height as i64;
        let max = *worker_times.iter().max().unwrap() as i64;
        let (mut l, mut r) = (0i64, max * h * (h + 1) / 2);
        while l + 1 < r {
            let m = l.midpoint(r);
            if check(&worker_times, h, m) {
                r = m;
            } else {
                l = m;
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_number_of_seconds(4, vec![2, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(15, Solution::min_number_of_seconds(5, vec![1]));
    }
}

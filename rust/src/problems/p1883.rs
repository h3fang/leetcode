pub struct Solution;

impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        if dist.iter().sum::<i32>() > speed * hours_before {
            return -1;
        }
        let n = dist.len();
        let mut f = vec![0; n + 1];
        for i in 0..n {
            let mut pre = 0;
            for (j, &d) in dist[..n - 1].iter().enumerate() {
                let tmp = f[j + 1];
                f[j + 1] = ((f[j] + d + speed - 1) / speed) * speed;
                if i > 0 {
                    f[j + 1] = f[j + 1].min(pre + d);
                }
                pre = tmp;
            }
            if f[n - 1] + dist[n - 1] <= speed * hours_before {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_skips(vec![1, 3, 2], 4, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_skips(vec![7, 3, 5, 5], 2, 10));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_skips(vec![7, 3, 5, 5], 1, 10));
    }
}

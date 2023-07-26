pub struct Solution;

fn time(dist: &[i32], speed: f64) -> f64 {
    dist.iter().fold(0.0, |t, &d| t.ceil() + d as f64 / speed)
}

fn upper_bound(dist: &[i32], hour: f64) -> i32 {
    let total = dist.iter().fold(0.0, |t, &d| t + d as f64);
    let t = hour + 1.0 - (dist.len() as f64);
    (total / t).ceil() as i32
}

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if (dist.len() - 1) as f64 >= hour {
            return -1;
        }

        let (mut l, mut r) = (1, upper_bound(&dist, hour));

        while l < r {
            let m = l + (r - l) / 2;
            if time(&dist, m as f64) > hour {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_speed_on_time(vec![1, 3, 2], 6.0));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_speed_on_time(vec![1, 3, 2], 2.7));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_speed_on_time(vec![1, 3, 2], 1.9));
    }
}

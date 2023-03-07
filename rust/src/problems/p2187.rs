pub struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        fn trips(time: &[i32], t: i64) -> i64 {
            time.iter().fold(0, |acc, &e| acc + t / (e as i64))
        }
        let total_trips = total_trips as i64;
        let mut left = 1;
        let mut right = (*time.iter().min().unwrap() as i64) * total_trips;
        while left < right {
            let mid = left + (right - left) / 2;
            let t = trips(&time, mid);
            match t.cmp(&total_trips) {
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                }
                _ => {
                    right = mid;
                }
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
        assert_eq!(3, Solution::minimum_time(vec![1, 2, 3], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_time(vec![2], 1));
    }
}

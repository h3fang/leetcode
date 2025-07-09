pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let (k, n) = (k as usize, start_time.len());
        let empty = |i: usize| {
            if i == 0 {
                start_time[0]
            } else if i == n {
                event_time - end_time[i - 1]
            } else {
                start_time[i] - end_time[i - 1]
            }
        };
        let (mut ans, mut curr) = (0, 0);
        for i in 0..=n {
            curr += empty(i);
            if i < k {
                continue;
            }
            ans = ans.max(curr);
            curr -= empty(i - k);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_free_time(5, 1, vec![1, 3], vec![2, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            6,
            Solution::max_free_time(10, 1, vec![0, 2, 9], vec![1, 4, 10])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::max_free_time(5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5])
        );
    }
}

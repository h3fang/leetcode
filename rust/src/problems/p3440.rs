pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let (mut ans, mut max) = (0, 0);
        for (i, (s, e)) in start_time.iter().zip(&end_time).enumerate() {
            let l = if i == 0 { 0 } else { end_time[i - 1] };
            let r = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };
            if e - s <= max {
                ans = ans.max(r - l);
            }
            max = max.max(s - l);
            ans = ans.max(r - l - (e - s));
        }
        max = 0;
        for (i, (s, e)) in start_time.iter().zip(&end_time).enumerate().rev() {
            let l = if i == 0 { 0 } else { end_time[i - 1] };
            let r = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };
            if e - s <= max {
                ans = ans.max(r - l);
            }
            max = max.max(r - e);
            ans = ans.max(r - l - (e - s));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_free_time(5, vec![1, 3], vec![2, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            7,
            Solution::max_free_time(10, vec![0, 7, 9], vec![1, 8, 10])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            6,
            Solution::max_free_time(10, vec![0, 3, 7, 9], vec![1, 4, 8, 10])
        );
    }
}

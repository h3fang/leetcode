pub struct Solution;

impl Solution {
    pub fn max_two_events_dp(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| e[1]);
        let mut f = vec![0; events.len()];
        let mut s = vec![0; events.len()];
        f[0] = events[0][2];
        for (i, e) in events.iter().enumerate().skip(1) {
            f[i] = f[i - 1].max(e[2]);
            let mut left = 0;
            let mut right = i as i64 - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                match events[mid as usize][1].cmp(&e[0]) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    _ => right = mid - 1,
                }
            }
            s[i] = s[i - 1];
            if right >= 0 {
                s[i] = s[i].max(f[right as usize] + e[2]);
            }
        }
        f[events.len() - 1].max(s[events.len() - 1])
    }

    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut timepoints = events
            .iter()
            .map(|e| [(e[0], 0, e[2]), (e[1], 1, e[2])])
            .flatten()
            .collect::<Vec<_>>();
        timepoints.sort_unstable();
        let mut best_first = 0;
        let mut best_second = 0;
        for t in timepoints {
            if t.1 == 0 {
                best_second = best_second.max(best_first + t.2);
            } else {
                best_first = best_first.max(t.2);
            }
        }
        best_second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let events = [[1, 3, 2], [4, 5, 2], [2, 4, 3]];
        let events = events.iter().map(|e| e.to_vec()).collect::<Vec<_>>();
        assert_eq!(4, Solution::max_two_events_dp(events.clone()));
        assert_eq!(4, Solution::max_two_events(events));
    }

    #[test]
    fn case2() {
        let events = [[1, 3, 2], [4, 5, 2], [1, 5, 5]];
        let events = events.iter().map(|e| e.to_vec()).collect::<Vec<_>>();
        assert_eq!(5, Solution::max_two_events_dp(events.clone()));
        assert_eq!(5, Solution::max_two_events(events));
    }

    #[test]
    fn case3() {
        let events = [[1, 5, 3], [1, 5, 1], [6, 6, 5]];
        let events = events.iter().map(|e| e.to_vec()).collect::<Vec<_>>();
        assert_eq!(8, Solution::max_two_events_dp(events.clone()));
        assert_eq!(8, Solution::max_two_events(events));
    }
}

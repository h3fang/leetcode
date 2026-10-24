pub struct Solution;

const DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn index_of_day(md: &str) -> usize {
    let month = md[..2].parse::<usize>().unwrap() - 1;
    let day = md[3..=4].parse::<usize>().unwrap() - 1;
    DAYS[..month].iter().sum::<i32>() as usize + day
}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let a0 = index_of_day(&arrive_alice);
        let a1 = index_of_day(&leave_alice);

        let b0 = index_of_day(&arrive_bob);
        let b1 = index_of_day(&leave_bob);

        let l = a0.max(b0) as i32;
        let r = a1.min(b1) as i32;

        (r - l + 1).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arrive_alice = "08-15".to_string();
        let leave_alice = "08-18".to_string();
        let arrive_bob = "08-16".to_string();
        let leave_bob = "08-19".to_string();
        assert_eq!(
            3,
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob)
        );
    }

    #[test]
    fn case2() {
        let arrive_alice = "10-01".to_string();
        let leave_alice = "10-31".to_string();
        let arrive_bob = "11-01".to_string();
        let leave_bob = "12-31".to_string();
        assert_eq!(
            0,
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob)
        );
    }
}

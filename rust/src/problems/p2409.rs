pub struct Solution;

const DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        fn index_of_day(md: &str) -> usize {
            let month = md[..2].parse::<usize>().unwrap() - 1;
            let day = md[3..=4].parse::<usize>().unwrap() - 1;
            DAYS[..month].iter().sum::<i32>() as usize + day
        }
        let mut mask = [0u8; 365];
        let a0 = index_of_day(&arrive_alice);
        let a1 = index_of_day(&leave_alice);
        mask[a0..=a1].iter_mut().for_each(|m| *m += 1);

        let a0 = index_of_day(&arrive_bob);
        let a1 = index_of_day(&leave_bob);
        mask[a0..=a1].iter_mut().for_each(|m| *m += 1);

        mask.iter().filter(|m| **m == 2).count() as i32
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
}

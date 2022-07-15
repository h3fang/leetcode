pub struct Solution;

impl Solution {
    pub fn latest_time_catch_the_bus(
        mut buses: Vec<i32>,
        mut passengers: Vec<i32>,
        capacity: i32,
    ) -> i32 {
        buses.sort_unstable();
        passengers.sort_unstable();
        let mut i = 0;
        let mut prev = 0;
        let mut result = 0;
        for b in buses {
            let mut c = capacity;
            while i < passengers.len() && passengers[i] <= b {
                if prev != passengers[i] - 1 {
                    result = passengers[i] - 1;
                }
                prev = passengers[i];
                c -= 1;
                i += 1;
                if c == 0 {
                    break;
                }
            }
            if c > 0 && prev != b {
                result = b;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            16,
            Solution::latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            20,
            Solution::latest_time_catch_the_bus(
                vec![20, 30, 10],
                vec![19, 13, 26, 4, 25, 11, 21],
                2
            )
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            11,
            Solution::latest_time_catch_the_bus(
                vec![18, 8, 3, 12, 9, 2, 7, 13, 20, 5],
                vec![13, 10, 8, 4, 12, 14, 18, 19, 5, 2, 30, 34],
                1
            )
        );
    }
}

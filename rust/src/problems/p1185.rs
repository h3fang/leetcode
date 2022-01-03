pub struct Solution;

const MONTH: [i32; 12] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut days = day + 3;
        for y in 1971..year {
            days += 365;
            if (y % 400 == 0) || (y % 4 == 0 && y % 100 != 0) {
                days += 1;
            }
        }
        for m in 1..month {
            days += MONTH[m as usize];
        }
        if month > 2 && ((year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)) {
            days += 1;
        }
        [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ][(days % 7) as usize]
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("Saturday", Solution::day_of_the_week(31, 8, 2019));
    }

    #[test]
    fn case2() {
        assert_eq!("Sunday", Solution::day_of_the_week(18, 7, 1999));
    }

    #[test]
    fn case3() {
        assert_eq!("Friday", Solution::day_of_the_week(1, 1, 1971));
    }
}

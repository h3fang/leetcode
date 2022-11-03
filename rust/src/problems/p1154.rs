pub struct Solution;

const DAYS: [i32; 12] = [
    0,
    31,
    31 + 28,
    31 + 28 + 31,
    31 + 28 + 31 + 30,
    31 + 28 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,
];

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let mut parts = date.split('-');
        let year = parts.next().unwrap().parse::<i32>().unwrap();
        let month = parts.next().unwrap().parse::<i32>().unwrap();
        let day = parts.next().unwrap().parse::<i32>().unwrap();
        let leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        DAYS[month as usize - 1] + day + i32::from(month >= 3 && leap_year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::day_of_year("2019-01-09".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(41, Solution::day_of_year("2019-02-10".into()));
    }

    #[test]
    fn case3() {
        assert_eq!(61, Solution::day_of_year("2004-03-01".into()));
    }
}

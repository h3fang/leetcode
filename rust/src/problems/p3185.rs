pub struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut f = [0; 24];
        let mut result = 0;
        for x in hours {
            let x = (x % 24) as usize;
            result += f[(24 - x) % 24];
            f[x] += 1;
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
            2,
            Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::count_complete_day_pairs(vec![72, 48, 24, 3]));
    }
}

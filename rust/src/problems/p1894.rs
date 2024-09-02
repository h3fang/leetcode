pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let sum: i64 = chalk.iter().map(|&c| c as i64).sum();
        let mut k = (k as i64 % sum) as i32;
        for (i, &c) in chalk.iter().enumerate() {
            if k < c {
                return i as i32;
            }
            k -= c;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::chalk_replacer(vec![5, 1, 5], 22));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::chalk_replacer(vec![3, 4, 1, 2], 25));
    }
}

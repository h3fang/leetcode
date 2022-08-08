use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let space = space as i64;
        let mut days = 0;
        let mut cd = HashMap::new();
        for t in tasks {
            let e = cd.entry(t).or_insert(i64::MIN);
            if *e + space + 1 > days {
                days = *e + space + 1;
            }
            *e = days;
            days += 1;
        }
        days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::task_scheduler_ii(vec![5, 8, 8, 5], 2));
    }
}

pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let m = tasks.len() as i32;
        let mut f = [0; 26];
        for c in tasks {
            f[(c as u8 - b'A') as usize] += 1;
        }
        let max = *f.iter().max().unwrap();
        let count = f.iter().filter(|&&e| e == max).count() as i32;
        let slots = (max - 1) * (n - count + 1);
        let remaining = m - count * max;
        let idles = (slots - remaining).max(0);
        m + idles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tasks = ["A", "A", "A", "B", "B", "B"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect();
        assert_eq!(8, Solution::least_interval(tasks, 2));
    }

    #[test]
    fn case2() {
        let tasks = ["A", "A", "A", "B", "B", "B"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect();
        assert_eq!(6, Solution::least_interval(tasks, 0));
    }

    #[test]
    fn case3() {
        let tasks = ["A", "A", "A", "A", "A", "A", "B", "C", "D", "E", "F", "G"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect();
        assert_eq!(16, Solution::least_interval(tasks, 2));
    }
}

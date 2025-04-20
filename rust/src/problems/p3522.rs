pub struct Solution;

impl Solution {
    pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
        let n = instructions.len() as i32;
        let mut vis = vec![false; n as usize];
        let (mut ans, mut i) = (0, 0);
        while i >= 0 && i < n && !vis[i as usize] {
            let j = i as usize;
            if instructions[j].as_bytes()[0] == b'j' {
                i += values[j];
            } else {
                ans += values[j] as i64;
                i += 1;
            }
            vis[j] = true;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let instructions = ["jump", "add", "add", "jump", "add", "jump"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let values = vec![2, 1, 3, 1, -2, -3];
        assert_eq!(1, Solution::calculate_score(instructions, values));
    }

    #[test]
    fn case2() {
        let instructions = ["jump", "add", "add"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let values = vec![3, 1, 1];
        assert_eq!(0, Solution::calculate_score(instructions, values));
    }

    #[test]
    fn case3() {
        let instructions = ["jump"].iter().map(|s| s.to_string()).collect();
        let values = vec![0];
        assert_eq!(0, Solution::calculate_score(instructions, values));
    }
}

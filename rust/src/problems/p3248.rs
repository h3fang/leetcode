pub struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut pos = 0;
        for c in commands {
            match c.as_str() {
                "LEFT" => pos -= 1,
                "RIGHT" => pos += 1,
                "UP" => pos -= n,
                "DOWN" => pos += n,
                _ => unreachable!(),
            }
        }
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let commands = ["RIGHT", "DOWN"].iter().map(|c| c.to_string()).collect();
        assert_eq!(3, Solution::final_position_of_snake(2, commands));
    }

    #[test]
    fn case2() {
        let commands = ["DOWN", "RIGHT", "UP"]
            .iter()
            .map(|c| c.to_string())
            .collect();
        assert_eq!(1, Solution::final_position_of_snake(3, commands));
    }
}

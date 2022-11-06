pub struct Solution;

impl Solution {
    pub fn interpret(mut command: String) -> String {
        command = command.replace("()", "o");
        command = command.replace("(al)", "al");
        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("Goal", Solution::interpret("G()(al)".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("Gooooal", Solution::interpret("G()()()()(al)".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            "alGalooG",
            Solution::interpret("(al)G(al)()()G".to_string())
        );
    }
}

pub struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut curr = 1;
        for t in target {
            while curr < t {
                result.push("Push".to_string());
                result.push("Pop".to_string());
                curr += 1;
            }
            result.push("Push".to_string());
            curr += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operate(ops: &[String]) -> Vec<i32> {
        let mut result = vec![];
        let mut curr = 1;
        for op in ops {
            if op == "Push" {
                result.push(curr);
                curr += 1;
            } else {
                result.pop();
            }
        }
        result
    }

    fn validate(target: Vec<i32>, n: i32) {
        let ops = Solution::build_array(target.clone(), n);
        let arr = operate(&ops);
        assert_eq!(target, arr);
    }

    #[test]
    fn case1() {
        validate(vec![1, 3], 3);
    }

    #[test]
    fn case2() {
        validate(vec![1, 2, 3], 3);
    }

    #[test]
    fn case3() {
        validate(vec![1, 2], 4);
    }
}

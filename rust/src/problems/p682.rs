pub struct Solution;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut scores = vec![];
        for op in ops {
            match op.as_str() {
                "+" => {
                    let n = scores.len();
                    let s = scores[n - 1] + scores[n - 2];
                    scores.push(s);
                }
                "D" => {
                    let s = scores[scores.len() - 1] * 2;
                    scores.push(s);
                }
                "C" => {
                    scores.pop();
                }
                num => {
                    scores.push(num.parse().unwrap());
                }
            }
        }
        scores.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let ops = ["5", "2", "C", "D", "+"];
        let ops = ops.iter().map(|op| op.to_string()).collect();
        assert_eq!(30, Solution::cal_points(ops));
    }

    #[test]
    fn case2() {
        let ops = ["5", "-2", "4", "C", "D", "9", "+", "+"];
        let ops = ops.iter().map(|op| op.to_string()).collect();
        assert_eq!(27, Solution::cal_points(ops));
    }

    #[test]
    fn case3() {
        let ops = ["1"];
        let ops = ops.iter().map(|op| op.to_string()).collect();
        assert_eq!(1, Solution::cal_points(ops));
    }
}

pub struct Solution;

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut result = 0.0;
        let mut last = 0;
        for b in brackets {
            if income >= last {
                result += (income.min(b[0]) - last) as f64 * (b[1] as f64 / 100.0);
                last = b[0];
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let brackets = [[3, 50], [7, 10], [12, 25]];
        let brackets = brackets.iter().map(|b| b.to_vec()).collect();
        let income = 10;
        assert!((2.65 - Solution::calculate_tax(brackets, income)).abs() <= 1e-5);
    }
}

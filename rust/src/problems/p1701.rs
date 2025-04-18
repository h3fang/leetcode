pub struct Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let n = customers.len() as f64;
        let (mut t, mut wait) = (0, 0);
        for c in customers {
            t = t.max(c[0]) + c[1];
            wait += (t - c[0]) as u64;
        }
        wait as f64 / n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5);
    }

    #[test]
    fn case1() {
        let customers = [[1, 2], [2, 5], [4, 3]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_close(5.0, Solution::average_waiting_time(customers));
    }

    #[test]
    fn case2() {
        let customers = [[5, 2], [5, 4], [10, 3], [20, 1]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_close(3.25, Solution::average_waiting_time(customers));
    }
}

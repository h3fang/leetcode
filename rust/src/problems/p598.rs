pub struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut a = m;
        let mut b = n;
        for ops in ops {
            a = a.min(ops[0]);
            b = b.min(ops[1]);
        }

        a * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let m = 3;
        let n = 3;
        let ops = vec![vec![2, 2], vec![3, 3]];
        assert_eq!(4, Solution::max_count(m, n, ops));
    }
}

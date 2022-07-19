pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(num_rows as usize);
        result.push(vec![1]);
        for i in 2..=num_rows {
            let p = result.last().unwrap();
            let mut r = Vec::with_capacity(i as usize);
            r.push(1);
            for j in 1..i as usize - 1 {
                r.push(p[j - 1] + p[j]);
            }
            r.push(1);
            result.push(r);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            Solution::generate(5)
        );
    }
}

pub struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        for (i, r) in matrix.iter().enumerate() {
            let min = r
                .iter()
                .enumerate()
                .min_by_key(|e| e.1)
                .map(|e| e.0)
                .unwrap();
            let max = matrix
                .iter()
                .map(|r| r[min])
                .enumerate()
                .max_by_key(|e| e.1)
                .map(|e| e.0)
                .unwrap();
            if max == i {
                result.push(r[min]);
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
        let matrix = [[3, 7, 8], [9, 11, 13], [15, 16, 17]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![15], Solution::lucky_numbers(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[1, 10, 4, 2], [9, 3, 8, 7], [15, 16, 17, 12]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![12], Solution::lucky_numbers(matrix));
    }
}

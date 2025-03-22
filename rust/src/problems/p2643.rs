pub struct Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut idx, mut ones) = (0, 0);
        for (i, r) in mat.iter().enumerate() {
            let c = r.iter().sum();
            if c > ones {
                (idx, ones) = (i as i32, c);
            }
        }
        vec![idx, ones]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[0, 1], [1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![0, 1], Solution::row_and_maximum_ones(mat));
    }

    #[test]
    fn case2() {
        let mat = [[0, 0, 0], [0, 1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![1, 2], Solution::row_and_maximum_ones(mat));
    }

    #[test]
    fn case3() {
        let mat = [[0, 0], [1, 1], [0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(vec![1, 2], Solution::row_and_maximum_ones(mat));
    }
}

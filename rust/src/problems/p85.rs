pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        if n == 0 {
            return 0;
        }

        let mut result = 0;
        let mut heights = vec![0; n + 1];
        for i in 0..m {
            let mut stack = vec![(-1i32, -1i32)];
            for j in 0..=n {
                if j < n {
                    if matrix[i][j] == '1' {
                        heights[j] += 1;
                    } else {
                        heights[j] = 0;
                    }
                }

                let h = heights[j];
                while h < stack.last().unwrap().1 {
                    let hi = stack.pop().unwrap().1;
                    let width = j as i32 - stack.last().unwrap().0 - 1;
                    result = result.max(hi * width);
                }
                stack.push((j as i32, h));
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ];
        let matrix = matrix
            .iter()
            .map(|r| {
                r.iter()
                    .map(|s| s.parse::<char>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(6, Solution::maximal_rectangle(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [
            ["0", "0", "0", "0", "0", "0", "1"],
            ["0", "0", "0", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1", "1", "1"],
            ["0", "0", "0", "1", "1", "1", "1"],
        ];
        let matrix = matrix
            .iter()
            .map(|r| {
                r.iter()
                    .map(|s| s.parse::<char>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(9, Solution::maximal_rectangle(matrix));
    }

    #[test]
    fn case3() {
        let matrix = [
            ["1", "0", "0", "0", "1"],
            ["1", "1", "0", "1", "1"],
            ["1", "1", "1", "1", "1"],
        ];
        let matrix = matrix
            .iter()
            .map(|r| {
                r.iter()
                    .map(|s| s.parse::<char>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(5, Solution::maximal_rectangle(matrix));
    }
}

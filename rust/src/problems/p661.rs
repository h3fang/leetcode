pub struct Solution;

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = img.clone();
        let height = img.len();
        let width = img[0].len();
        for (i, row) in result.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                let mut c = 0;
                let mut sum = 0;
                for row in img
                    .iter()
                    .take((i + 1).min(height - 1) + 1)
                    .skip(i.saturating_sub(1))
                {
                    for p in row
                        .iter()
                        .take((j + 1).min(width - 1) + 1)
                        .skip(j.saturating_sub(1))
                    {
                        sum += p;
                        c += 1;
                    }
                }
                *pixel = sum / c;
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
        let img = [[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        let img = img.iter().map(|r| r.to_vec()).collect();
        let expected = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::image_smoother(img));
    }

    #[test]
    fn case2() {
        let img = [[100, 200, 100], [200, 50, 200], [100, 200, 100]];
        let img = img.iter().map(|r| r.to_vec()).collect();
        let expected = [[137, 141, 137], [141, 138, 141], [137, 141, 137]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::image_smoother(img));
    }
}

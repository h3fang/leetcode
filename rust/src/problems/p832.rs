pub struct Solution;

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut image = image;
        image.iter_mut().for_each(|row| {
            row.reverse();
            row.iter_mut().for_each(|p| *p = 1 - *p)
        });
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let image = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let expected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        assert_eq!(expected, Solution::flip_and_invert_image(image));
    }

    #[test]
    fn case2() {
        let image = vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
        ];
        let expected = vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0],
        ];
        assert_eq!(expected, Solution::flip_and_invert_image(image));
    }
}

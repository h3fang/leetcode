pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn bt(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, old_color: i32, new_color: i32) {
            if sr < 0
                || sc < 0
                || sr >= image.len() as i32
                || sc >= image[0].len() as i32
                || old_color == new_color
            {
                return;
            }

            if image[sr as usize][sc as usize] == old_color {
                image[sr as usize][sc as usize] = new_color;
                bt(image, sr - 1, sc, old_color, new_color);
                bt(image, sr + 1, sc, old_color, new_color);
                bt(image, sr, sc - 1, old_color, new_color);
                bt(image, sr, sc + 1, old_color, new_color);
            }
        }

        let old_color = image[sr as usize][sc as usize];
        bt(&mut image, sr, sc, old_color, new_color);

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let image = [[1, 1, 1], [1, 1, 0], [1, 0, 1]];
        let image = image.iter().map(|r| r.to_vec()).collect::<Vec<_>>();

        let expected = [[2, 2, 2], [2, 2, 0], [2, 0, 1]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::flood_fill(image, 1, 1, 2));
    }
}

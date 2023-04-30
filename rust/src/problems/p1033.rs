pub struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let x = a.min(b.min(c));
        let z = a.max(b.max(c));
        let y = a + b + c - x - z;
        let min = if y - x == 1 && z - y == 1 {
            0
        } else if z - y <= 2 || y - x <= 2 {
            1
        } else {
            2
        };
        vec![min, z - x - 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 2], Solution::num_moves_stones(1, 2, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0, 0], Solution::num_moves_stones(4, 3, 2));
    }
}

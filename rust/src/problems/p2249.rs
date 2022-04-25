use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let mut pts = HashSet::new();
        for c in circles {
            for x in c[0] - c[2]..=c[0] + c[2] {
                for y in c[1] - c[2]..=c[1] + c[2] {
                    if (x - c[0]) * (x - c[0]) + (y - c[1]) * (y - c[1]) <= c[2] * c[2] {
                        pts.insert((x, y));
                    }
                }
            }
        }
        pts.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let circles = vec![vec![2, 2, 1]];
        assert_eq!(5, Solution::count_lattice_points(circles));
    }

    #[test]
    fn case2() {
        let circles = vec![vec![2, 2, 2], vec![3, 4, 1]];
        assert_eq!(16, Solution::count_lattice_points(circles));
    }
}

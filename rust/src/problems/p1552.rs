pub struct Solution;

fn valid(positions: &[i32], mut m: i32, d: i32) -> bool {
    let mut x = positions[0];
    m -= 1;
    for &y in positions {
        if y - x >= d {
            m -= 1;
            if m == 0 {
                return true;
            }
            x = y;
        }
    }
    false
}

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let (mut l, mut r) = (0, position.last().unwrap() - position[0]);
        while l < r {
            let d = (r - l + 1) / 2 + l;
            if valid(&position, m, d) {
                l = d;
            } else {
                r = d - 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_distance(vec![1, 2, 3, 4, 7], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(
            999999999,
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2)
        );
    }
}

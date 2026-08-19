pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut reserved: HashMap<i32, u16> = HashMap::new();
        for s in reserved_seats {
            if (2..=9).contains(&s[1]) {
                let e = reserved.entry(s[0]).or_default();
                *e |= 1 << s[1];
            }
        }

        let mut rev = 0;
        for &seat in reserved.values() {
            if [0b1111000000, 0b11110000, 0b111100]
                .into_iter()
                .any(|m| seat & m == 0)
            {
                rev += 1;
            } else {
                rev += 2;
            }
        }
        2 * n - rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let reserved_seats = [[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        assert_eq!(4, Solution::max_number_of_families(3, reserved_seats));
    }

    #[test]
    fn case2() {
        let reserved_seats = [[2, 1], [1, 8], [2, 6]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        assert_eq!(2, Solution::max_number_of_families(2, reserved_seats));
    }

    #[test]
    fn case3() {
        let reserved_seats = [[4, 3], [1, 4], [4, 6], [1, 7]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        assert_eq!(4, Solution::max_number_of_families(4, reserved_seats));
    }
}

pub struct Solution;

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (skills.len(), k as usize);
        let mut i = 0;
        'outer: while i < n {
            let matches = if i == 0 { k } else { k - 1 };
            for d in 1..=matches {
                if i + d == n {
                    return i as i32;
                }
                if skills[i + d] > skills[i] {
                    i += d;
                    continue 'outer;
                }
            }
            return i as i32;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_winning_player(vec![2, 5, 4], 3));
    }
}

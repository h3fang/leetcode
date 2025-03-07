pub struct Solution;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let max = *arr.iter().max().unwrap();
        let mut a = arr[0];
        let mut wins = 0;
        for &b in &arr[1..] {
            if a > b {
                wins += 1;
            } else {
                wins = 1;
                a = b;
            }
            if wins == k || a == max {
                return a;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::get_winner(vec![3, 2, 1], 10));
    }
}

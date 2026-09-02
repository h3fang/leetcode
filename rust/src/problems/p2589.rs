pub struct Solution;

impl Solution {
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|t| t[1]);
        let mut time = [false; 2001];
        for t in tasks {
            let mut r = t[2]
                - time[t[0] as usize..=t[1] as usize]
                    .iter()
                    .filter(|&&e| e)
                    .count() as i32;
            for e in time[t[0] as usize..=t[1] as usize].iter_mut().rev() {
                if r <= 0 {
                    break;
                }
                if !*e {
                    *e = true;
                    r -= 1;
                }
            }
        }
        time.into_iter().filter(|&e| e).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tasks = [[2, 3, 1], [4, 5, 1], [1, 5, 2]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(2, Solution::find_minimum_time(tasks));
    }

    #[test]
    fn case2() {
        let tasks = [[1, 3, 2], [2, 5, 3], [5, 6, 2]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(4, Solution::find_minimum_time(tasks));
    }
}

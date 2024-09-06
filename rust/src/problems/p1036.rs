use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        fn distance(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
            (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
        }

        fn is_valid(&(x, y): &(i32, i32)) -> bool {
            x >= 0 && y >= 0 && x < 100_0000 && y < 100_0000
        }

        fn is_open(
            source: (i32, i32),
            target: (i32, i32),
            curr: (i32, i32),
            visited: &mut HashSet<(i32, i32)>,
            blocked: &HashSet<(i32, i32)>,
        ) -> bool {
            if curr == target || distance(&source, &curr) > 200 {
                return true;
            }
            visited.insert(curr);
            for next in [
                (curr.0 + 1, curr.1),
                (curr.0 - 1, curr.1),
                (curr.0, curr.1 + 1),
                (curr.0, curr.1 - 1),
            ] {
                if !is_valid(&next) || visited.contains(&next) || blocked.contains(&next) {
                    continue;
                }
                if is_open(source, target, next, visited, blocked) {
                    return true;
                }
            }
            false
        }

        let source = (source[0], source[1]);
        let target = (target[0], target[1]);
        let blocked = blocked
            .into_iter()
            .map(|p| (p[0], p[1]))
            .collect::<HashSet<(i32, i32)>>();
        is_open(source, target, source, &mut HashSet::new(), &blocked)
            && is_open(target, source, target, &mut HashSet::new(), &blocked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let blocked = vec![vec![0, 1], vec![1, 0]];
        let source = vec![0, 0];
        let target = vec![0, 2];
        assert!(!Solution::is_escape_possible(blocked, source, target));
    }

    #[test]
    fn case2() {
        let blocked = vec![vec![0i32; 2]; 0];
        let source = vec![0, 0];
        let target = vec![999999, 999999];
        assert!(Solution::is_escape_possible(blocked, source, target));
    }
}

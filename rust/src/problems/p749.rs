pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        fn find_virus(g: &mut [Vec<i32>]) -> Vec<(usize, i32, i32)> {
            let mut result = vec![];
            let m = g.len() as i8;
            let n = g[0].len() as i8;
            for i in 0..m {
                for j in 0..n {
                    if g[i as usize][j as usize] != 1 {
                        continue;
                    }
                    let idx = result.len() as i32 + 1;
                    let mut outside = HashSet::new();
                    let mut edge = 0;
                    let mut q = VecDeque::new();
                    q.push_back((i as i8, j as i8));
                    while let Some((i, j)) = q.pop_front() {
                        if g[i as usize][j as usize] != 1 {
                            if g[i as usize][j as usize] == 0 {
                                outside.insert((i, j));
                                edge += 1;
                            }
                            continue;
                        }
                        g[i as usize][j as usize] = -idx;
                        for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                            if i1 < 0 || j1 < 0 || i1 == m || j1 == n {
                                continue;
                            }
                            q.push_back((i1, j1));
                        }
                    }
                    result.push((outside.len(), edge, idx));
                }
            }
            result.sort_unstable_by_key(|e| e.0);
            result
        }

        fn disinfect(g: &mut [Vec<i32>], idx: i32) {
            g.iter_mut().flatten().for_each(|c| {
                if *c == -idx {
                    *c = 2;
                }
            })
        }

        fn spread(g: &mut [Vec<i32>]) {
            g.iter_mut().flatten().for_each(|c| {
                if *c < 0 {
                    *c = 1;
                }
            });
            let m = g.len();
            let n = g[0].len();
            for i in 0..m {
                for j in 0..n {
                    if g[i][j] == 1 {
                        let i = i as i8;
                        let j = j as i8;
                        for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                            if i1 < 0 || j1 < 0 || i1 == m as i8 || j1 == n as i8 {
                                continue;
                            }
                            if g[i1 as usize][j1 as usize] == 0 {
                                g[i1 as usize][j1 as usize] = 3;
                            }
                        }
                    }
                }
            }
            g.iter_mut().flatten().for_each(|c| {
                if *c == 3 {
                    *c = 1;
                }
            });
        }

        let mut result = 0;
        loop {
            let vs = find_virus(&mut is_infected);
            if vs.is_empty() {
                break;
            }
            let a = vs.last().unwrap();
            result += a.1;
            disinfect(&mut is_infected, a.2);
            spread(&mut is_infected);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let is_infected = [
            [0, 1, 0, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let is_infected = is_infected.iter().map(|r| r.to_vec()).collect();
        assert_eq!(10, Solution::contain_virus(is_infected));
    }

    #[test]
    fn case2() {
        let is_infected = [[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        let is_infected = is_infected.iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::contain_virus(is_infected));
    }

    #[test]
    fn case3() {
        let is_infected = [
            [1, 1, 1, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1, 1],
            [1, 1, 1, 0, 0, 0, 0, 0, 0],
        ];
        let is_infected = is_infected.iter().map(|r| r.to_vec()).collect();
        assert_eq!(13, Solution::contain_virus(is_infected));
    }

    #[test]
    fn case4() {
        let is_infected = [
            [0, 1, 0, 1, 1, 1, 1, 1, 1, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0, 0, 1, 0],
            [0, 0, 0, 1, 1, 0, 0, 1, 1, 0],
            [0, 1, 0, 0, 1, 0, 1, 1, 0, 1],
            [0, 0, 0, 1, 0, 1, 0, 1, 1, 1],
            [0, 1, 0, 0, 1, 0, 0, 1, 1, 0],
            [0, 1, 0, 1, 0, 0, 0, 1, 1, 0],
            [0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
            [1, 0, 1, 1, 0, 1, 0, 1, 0, 1],
        ];
        let is_infected = is_infected.iter().map(|r| r.to_vec()).collect();
        assert_eq!(38, Solution::contain_virus(is_infected));
    }
}

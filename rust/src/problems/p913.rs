pub struct Solution;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            cache: &mut [Vec<Vec<u8>>],
            graph: &[Vec<i32>],
            mouse: u8,
            cat: u8,
            moves: u8,
        ) -> u8 {
            if mouse == 0 {
                return 1;
            } else if mouse == cat {
                return 2;
            } else if moves as usize == 2 * graph.len() {
                return 0;
            }
            let r = cache[mouse as usize][cat as usize][moves as usize];
            if r != u8::MAX {
                return r;
            }
            let mouse_move = moves % 2 == 0;
            let mut expect = 1;
            let mut result = 2;
            let node = if mouse_move {
                mouse
            } else {
                expect = 2;
                result = 1;
                cat
            };
            for &next in &graph[node as usize] {
                if !mouse_move && next == 0 {
                    continue;
                }
                let r = if mouse_move {
                    dfs(cache, graph, next as u8, cat, moves + 1)
                } else {
                    dfs(cache, graph, mouse, next as u8, moves + 1)
                };
                if r == expect {
                    result = r;
                    break;
                } else if r == 0 {
                    result = 0;
                }
            }
            cache[mouse as usize][cat as usize][moves as usize] = result;
            result
        }

        let n = graph.len();
        let mut cache = vec![vec![vec![u8::MAX; 2 * n]; n]; n];
        dfs(&mut cache, &graph, 1, 2, 0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let graph = vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3],
        ];
        assert_eq!(0, Solution::cat_mouse_game(graph));
    }

    #[test]
    fn case2() {
        let graph = vec![vec![1, 3], vec![0], vec![3], vec![0, 2]];
        assert_eq!(1, Solution::cat_mouse_game(graph));
    }

    #[test]
    fn case3() {
        let graph = vec![
            vec![5, 6],
            vec![3, 4],
            vec![6],
            vec![1, 4, 5],
            vec![1, 3, 5],
            vec![0, 3, 4, 6],
            vec![0, 2, 5],
        ];
        assert_eq!(2, Solution::cat_mouse_game(graph));
    }
}

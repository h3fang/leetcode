pub struct Solution;

const ROOK_DIRS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const BISHOP_DIRS: [(i8, i8); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

struct Move(i8, i8, i8);

impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        fn gen_moves((x, y): &(i8, i8), dirs: &[(i8, i8)]) -> [Move; 8] {
            const M: Move = Move(0, 0, 0);
            let mut r = [M; 8];
            for (i, (dx, dy)) in dirs.iter().enumerate() {
                r[i].0 = *dx;
                r[i].1 = *dy;
                for d in 1..8 {
                    let x1 = x + dx * d;
                    let y1 = y + dy * d;
                    if x1 > 0 && x1 <= 8 && y1 > 0 && y1 <= 8 {
                        r[i].2 = d;
                    } else {
                        break;
                    }
                }
            }
            r
        }

        fn is_valid(mut x1: i8, mut y1: i8, m1: &Move, mut x2: i8, mut y2: i8, m2: &Move) -> bool {
            let mut d = 0;
            while d < m1.2 || d < m2.2 {
                if d < m1.2 {
                    x1 += m1.0;
                    y1 += m1.1;
                }

                if d < m2.2 {
                    x2 += m2.0;
                    y2 += m2.1;
                }

                if x1 == x2 && y1 == y2 {
                    return false;
                }

                d += 1;
            }
            true
        }

        fn no_conflict(curr: &[Move], positions: &[(i8, i8)], x1: i8, y1: i8, m1: &Move) -> bool {
            let mut valid = true;
            for (j, m2) in curr.iter().enumerate() {
                let &(x2, y2) = &positions[j];
                if !is_valid(x1, y1, m1, x2, y2, m2) {
                    valid = false;
                    break;
                }
            }
            valid
        }

        fn dfs(
            positions: &[(i8, i8)],
            moves: &[[Move; 8]],
            curr: &mut Vec<Move>,
            result: &mut i32,
        ) {
            let i = curr.len();
            if i == moves.len() {
                *result += 1;
            } else {
                let &(x1, y1) = &positions[i];

                let m1 = Move(0, 0, 0);
                if no_conflict(curr, positions, x1, y1, &m1) {
                    curr.push(m1);
                    dfs(positions, moves, curr, result);
                    curr.pop();
                }

                for m in &moves[i] {
                    for d in 1..=m.2 {
                        let m1 = Move(m.0, m.1, d);
                        if no_conflict(curr, positions, x1, y1, &m1) {
                            curr.push(m1);
                            dfs(positions, moves, curr, result);
                            curr.pop();
                        }
                    }
                }
            }
        }

        let positions = positions
            .iter()
            .map(|p| (p[0] as i8, p[1] as i8))
            .collect::<Vec<_>>();

        let moves = pieces
            .iter()
            .zip(&positions)
            .map(|(piece, pos)| match piece.as_str() {
                "rook" => gen_moves(pos, &ROOK_DIRS),
                "bishop" => gen_moves(pos, &BISHOP_DIRS),
                _ => gen_moves(pos, &[BISHOP_DIRS, ROOK_DIRS].concat()),
            })
            .collect::<Vec<_>>();

        let mut curr = Vec::new();
        let mut result = 0;
        dfs(&positions, &moves, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let pieces = ["rook"];
        let pieces = pieces.iter().map(|p| p.to_string()).collect::<Vec<_>>();
        let positions = [[1, 1]];
        let positions = positions.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(15, Solution::count_combinations(pieces, positions));
    }

    #[test]
    fn case2() {
        let pieces = ["queen"];
        let pieces = pieces.iter().map(|p| p.to_string()).collect::<Vec<_>>();
        let positions = [[1, 1]];
        let positions = positions.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(22, Solution::count_combinations(pieces, positions));
    }

    #[test]
    fn case3() {
        let pieces = ["bishop"];
        let pieces = pieces.iter().map(|p| p.to_string()).collect::<Vec<_>>();
        let positions = [[4, 3]];
        let positions = positions.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(12, Solution::count_combinations(pieces, positions));
    }

    #[test]
    fn case4() {
        let pieces = ["rook", "rook"];
        let pieces = pieces.iter().map(|p| p.to_string()).collect::<Vec<_>>();
        let positions = [[1, 1], [8, 8]];
        let positions = positions.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(223, Solution::count_combinations(pieces, positions));
    }

    #[test]
    fn case5() {
        let pieces = ["queen", "bishop"];
        let pieces = pieces.iter().map(|p| p.to_string()).collect::<Vec<_>>();
        let positions = [[5, 7], [3, 4]];
        let positions = positions.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(281, Solution::count_combinations(pieces, positions));
    }
}

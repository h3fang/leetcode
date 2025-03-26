pub struct Solution;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let m = seats.len();
        let n = seats[0].len();
        let mut a = vec![0; m];
        for (i, row) in seats.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '.' {
                    a[i] |= 1 << j;
                }
            }
        }

        let mut f = vec![vec![0; 1 << n]; m];
        for j in 1..(1 << n) {
            let lb = (j as i32 & -(j as i32)) as usize;
            f[0][j] = f[0][j & !(lb * 3)] + 1;
        }
        for i in 1..m {
            let mut j = a[i];
            while j > 0 {
                f[i][j] = f[i - 1][a[i - 1]];
                let mut s = j;
                while s > 0 {
                    if (s & (s >> 1)) == 0 {
                        let t = a[i - 1] & !((s << 1) | (s >> 1));
                        f[i][j] = f[i][j].max(f[i - 1][t] + f[0][s]);
                    }
                    s = (s - 1) & j;
                }
                j = (j - 1) & a[i];
            }
            f[i][0] = f[i - 1][a[i - 1]];
        }
        f[m - 1][a[m - 1]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let seats = [
            ["#", ".", "#", "#", ".", "#"],
            [".", "#", "#", "#", "#", "."],
            ["#", ".", "#", "#", ".", "#"],
        ]
        .iter()
        .map(|r| r.iter().map(|e| e.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(4, Solution::max_students(seats));
    }

    #[test]
    fn case2() {
        let seats = [[".", "#"], ["#", "#"], ["#", "."], ["#", "#"], [".", "#"]]
            .iter()
            .map(|r| r.iter().map(|e| e.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(3, Solution::max_students(seats));
    }

    #[test]
    fn case3() {
        let seats = [
            ["#", ".", ".", ".", "#"],
            [".", "#", ".", "#", "."],
            [".", ".", "#", ".", "."],
            [".", "#", ".", "#", "."],
            ["#", ".", ".", ".", "#"],
        ]
        .iter()
        .map(|r| r.iter().map(|e| e.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(10, Solution::max_students(seats));
    }
}

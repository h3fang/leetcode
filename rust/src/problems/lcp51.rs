pub struct Solution;

struct Alg {
    mat: Vec<i32>,
    book: Vec<Vec<i32>>,
    attr: Vec<Vec<i32>>,
    limit: i32,
    result: i32,
}

impl Alg {
    fn dfs(&mut self, delicious: i32, satiety: i32, i: usize) {
        if satiety >= self.limit {
            self.result = self.result.max(delicious);
        }

        if i == self.book.len() {
            return;
        }

        self.dfs(delicious, satiety, i + 1);
        if self.mat.iter().zip(&self.book[i]).any(|(a, b)| a < b) {
            return;
        }
        for (a, b) in self.mat.iter_mut().zip(&self.book[i]) {
            *a -= b;
        }
        self.dfs(
            delicious + self.attr[i][0],
            satiety + self.attr[i][1],
            i + 1,
        );
        for (a, b) in self.mat.iter_mut().zip(&self.book[i]) {
            *a += b;
        }
    }
}

impl Solution {
    pub fn perfect_menu(
        materials: Vec<i32>,
        cookbooks: Vec<Vec<i32>>,
        attribute: Vec<Vec<i32>>,
        limit: i32,
    ) -> i32 {
        let mut alg = Alg {
            mat: materials,
            book: cookbooks,
            attr: attribute,
            limit,
            result: -1,
        };
        alg.dfs(0, 0, 0);
        alg.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let materials = vec![3, 2, 4, 1, 2];
        let cookbooks = vec![
            vec![1, 1, 0, 1, 2],
            vec![2, 1, 4, 0, 0],
            vec![3, 2, 4, 1, 0],
        ];
        let attribute = vec![vec![3, 2], vec![2, 4], vec![7, 6]];
        let limit = 5;
        assert_eq!(
            7,
            Solution::perfect_menu(materials, cookbooks, attribute, limit)
        );
    }

    #[test]
    fn case2() {
        let materials = vec![10, 10, 10, 10, 10];
        let cookbooks = vec![
            vec![1, 1, 1, 1, 1],
            vec![3, 3, 3, 3, 3],
            vec![10, 10, 10, 10, 10],
        ];
        let attribute = vec![vec![5, 5], vec![6, 6], vec![10, 10]];
        let limit = 1;
        assert_eq!(
            11,
            Solution::perfect_menu(materials, cookbooks, attribute, limit)
        );
    }

    #[test]
    fn case3() {
        let materials = vec![4, 20, 16, 17, 14];
        let cookbooks = vec![
            vec![3, 1, 6, 12, 18],
            vec![12, 7, 17, 19, 16],
            vec![18, 4, 7, 8, 0],
            vec![18, 9, 20, 16, 4],
            vec![17, 15, 4, 7, 15],
            vec![2, 8, 9, 3, 13],
            vec![20, 12, 0, 7, 17],
            vec![3, 2, 7, 5, 5],
        ];
        let attribute = vec![
            vec![13, 4],
            vec![7, 18],
            vec![3, 11],
            vec![6, 20],
            vec![9, 0],
            vec![13, 16],
            vec![18, 3],
            vec![2, 5],
        ];
        let limit = 1;
        assert_eq!(
            13,
            Solution::perfect_menu(materials, cookbooks, attribute, limit)
        );
    }
}

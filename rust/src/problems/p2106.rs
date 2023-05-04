pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        fn step(fruits: &[Vec<i32>], start_pos: i32, left: usize, right: usize) -> i32 {
            fruits[right][0] - fruits[left][0]
                + (fruits[right][0] - start_pos)
                    .abs()
                    .min((start_pos - fruits[left][0]).abs())
        }

        let mut sum = 0;
        let mut result = 0;
        let mut left = 0;
        for (right, f) in fruits.iter().enumerate() {
            sum += f[1];
            while left <= right && step(&fruits, start_pos, left, right) > k {
                sum -= fruits[left][1];
                left += 1;
            }
            result = result.max(sum);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let fruits = [[2, 8], [6, 3], [8, 6]];
        let fruits = fruits.iter().map(|f| f.to_vec()).collect::<Vec<_>>();
        assert_eq!(9, Solution::max_total_fruits(fruits, 5, 4));
    }

    #[test]
    fn case2() {
        let fruits = [[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]];
        let fruits = fruits.iter().map(|f| f.to_vec()).collect::<Vec<_>>();
        assert_eq!(14, Solution::max_total_fruits(fruits, 5, 4));
    }

    #[test]
    fn case3() {
        let fruits = [[0, 3], [6, 4], [8, 5]];
        let fruits = fruits.iter().map(|f| f.to_vec()).collect::<Vec<_>>();
        assert_eq!(0, Solution::max_total_fruits(fruits, 3, 2));
    }

    #[test]
    fn case4() {
        let fruits = [[200000, 10000]];
        let fruits = fruits.iter().map(|f| f.to_vec()).collect::<Vec<_>>();
        assert_eq!(10000, Solution::max_total_fruits(fruits, 0, 200000));
    }

    #[test]
    fn case5() {
        let fruits = [
            [0, 15],
            [3, 56],
            [4, 98],
            [5, 81],
            [7, 16],
            [8, 77],
            [9, 89],
            [12, 82],
            [13, 49],
            [14, 59],
            [17, 40],
            [18, 83],
            [19, 35],
            [20, 31],
            [21, 44],
            [22, 92],
            [25, 84],
            [26, 42],
            [29, 4],
            [33, 78],
            [35, 83],
            [36, 3],
            [37, 71],
            [41, 24],
            [44, 81],
            [45, 35],
            [46, 81],
            [48, 81],
            [50, 85],
            [52, 32],
            [53, 93],
            [54, 89],
            [55, 82],
            [56, 60],
            [59, 52],
            [62, 79],
            [63, 90],
            [64, 41],
            [66, 15],
            [68, 43],
            [69, 32],
            [70, 51],
            [71, 79],
            [75, 39],
            [76, 21],
            [78, 16],
            [79, 44],
            [80, 73],
            [81, 95],
            [83, 95],
            [85, 11],
            [87, 80],
            [88, 2],
            [90, 89],
            [99, 35],
            [100, 95],
        ];
        let fruits = fruits.iter().map(|f| f.to_vec()).collect::<Vec<_>>();
        assert_eq!(3102, Solution::max_total_fruits(fruits, 86, 107));
    }
}

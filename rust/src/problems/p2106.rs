pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let n = fruits.len() as i32;
        let curr = fruits.binary_search_by_key(&start_pos, |e| e[0]);
        let p = curr.unwrap_or_else(|e| e);
        let mut result = 0;

        // left then right
        let mut lv = 0;
        let mut rv = 0;
        let mut l = p as i32 - 1;
        let mut r = p as i32;
        while l >= 0 && (k >= 2 * (start_pos - fruits[l as usize][0])) {
            lv += fruits[l as usize][1];
            l -= 1;
        }
        l += 1;
        result = result.max(lv);
        while r < n && (fruits[r as usize][0] - start_pos) <= k {
            rv += fruits[r as usize][1];
            let left = k - (fruits[r as usize][0] - start_pos);
            r += 1;
            while 2 * (start_pos - fruits[l as usize][0]) > left {
                lv -= fruits[l as usize][1];
                l += 1;
            }
            result = result.max(lv + rv);
        }

        // right then left
        lv = 0;
        rv = 0;
        l = p as i32 - 1;
        r = p as i32;
        while r < n && (k >= 2 * (fruits[r as usize][0] - start_pos)) {
            rv += fruits[r as usize][1];
            r += 1;
        }
        r -= 1;
        result = result.max(rv);
        while l >= 0 && (start_pos - fruits[l as usize][0]) <= k {
            lv += fruits[l as usize][1];
            let right = k - (start_pos - fruits[l as usize][0]);
            l -= 1;
            while 2 * (fruits[r as usize][0] - start_pos) > right {
                rv -= fruits[r as usize][1];
                r -= 1;
            }
            result = result.max(lv + rv);
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

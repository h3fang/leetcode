use std::collections::HashMap;

#[derive(Default)]
pub struct DetectSquares {
    points: HashMap<i32, HashMap<i32, i32>>,
}

impl DetectSquares {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, point: Vec<i32>) {
        *self
            .points
            .entry(point[0])
            .or_default()
            .entry(point[1])
            .or_default() += 1;
    }

    fn count_points(&self, (x, y): (i32, i32)) -> i32 {
        self.points
            .get(&x)
            .map(|ys| ys.get(&y).cloned().unwrap_or(0))
            .unwrap_or(0)
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let x = point[0];
        let y = point[1];

        let mut result = 0;
        if let Some(ys) = self.points.get(&x) {
            for (&y1, n) in ys {
                let len = (y - y1).abs();
                if len == 0 {
                    continue;
                }
                result += n
                    * (self.count_points((x + len, y)) * self.count_points((x + len, y1))
                        + self.count_points((x - len, y)) * self.count_points((x - len, y1)));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut obj = DetectSquares::new();
        obj.add(vec![3, 10]);
        obj.add(vec![11, 2]);
        obj.add(vec![3, 2]);
        assert_eq!(1, obj.count(vec![11, 10]));
    }
}

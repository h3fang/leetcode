pub struct Robot {
    w: i32,
    h: i32,
    total: i32,
    p: i32,
    moved: bool,
}

impl Robot {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            w: width,
            h: height,
            total: 2 * width + 2 * height - 4,
            p: 0,
            moved: false,
        }
    }

    pub fn r#move(&mut self, num: i32) {
        self.p = (self.p + num) % self.total;
        self.moved = true;
    }

    pub fn get_pos(&self) -> Vec<i32> {
        if self.p < self.w {
            vec![self.p, 0]
        } else if self.p < self.w + self.h - 1 {
            vec![self.w - 1, self.p - (self.w - 1)]
        } else if self.p < 2 * self.w + self.h - 2 {
            vec![self.w - 1 - (self.p - (self.w + self.h - 2)), self.h - 1]
        } else {
            vec![0, self.h - 1 - (self.p - (2 * self.w + self.h - 3))]
        }
    }

    pub fn get_dir(&self) -> String {
        if !self.moved {
            "East".to_string()
        } else if self.p == 0 {
            "South".to_string()
        } else if self.p < self.w {
            "East".to_string()
        } else if self.p < self.w + self.h - 1 {
            "North".to_string()
        } else if self.p < 2 * self.w + self.h - 2 {
            "West".to_string()
        } else {
            "South".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut robot = Robot::new(6, 3);
        robot.r#move(2);
        robot.r#move(2);
        assert_eq!(vec![4, 0], robot.get_pos());
        assert_eq!("East".to_string(), robot.get_dir());
        robot.r#move(2);
        robot.r#move(1);
        robot.r#move(4);
        assert_eq!(vec![1, 2], robot.get_pos());
        assert_eq!("West".to_string(), robot.get_dir());
        robot.r#move(3);
        assert_eq!(vec![0, 0], robot.get_pos());
        assert_eq!("South".to_string(), robot.get_dir());
    }
}

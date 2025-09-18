use std::collections::{BinaryHeap, HashMap};

pub struct TaskManager {
    task: HashMap<i32, (i32, i32)>,
    priority: BinaryHeap<(i32, i32)>,
}

impl TaskManager {
    pub fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task = HashMap::with_capacity(tasks.len() * 2);
        let mut priority = BinaryHeap::with_capacity(tasks.len());
        for t in tasks {
            task.insert(t[1], (t[0], t[2]));
            priority.push((t[2], t[1]));
        }
        Self { task, priority }
    }

    pub fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task.insert(task_id, (user_id, priority));
        self.priority.push((priority, task_id));
    }

    pub fn edit(&mut self, task_id: i32, new_priority: i32) {
        self.task.get_mut(&task_id).unwrap().1 = new_priority;
        self.priority.push((new_priority, task_id));
    }

    pub fn rmv(&mut self, task_id: i32) {
        *self.task.get_mut(&task_id).unwrap() = (-1, -1);
    }

    pub fn exec_top(&mut self) -> i32 {
        while let Some((p, t)) = self.priority.pop() {
            let e = self.task.get_mut(&t).unwrap();
            if e.1 != p {
                continue;
            }
            let user = e.0;
            *e = (-1, -1);
            return user;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut tm = TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
        tm.add(4, 104, 5);
        tm.edit(102, 8);
        assert_eq!(3, tm.exec_top());
        tm.rmv(101);
        tm.add(5, 105, 15);
        assert_eq!(5, tm.exec_top());
    }
}

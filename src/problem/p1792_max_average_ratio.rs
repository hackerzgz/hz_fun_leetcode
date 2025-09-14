use core::f64;
use std::collections::BinaryHeap;

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
            0.78333
        );

        assert_eq!(
            Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4),
            0.53485
        );
    }
}

#[derive(PartialEq, Debug)]
struct ClassRatio {
    pass: i64,
    total: i64,
}

impl Eq for ClassRatio {}

impl PartialOrd for ClassRatio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let val1 = (other.total + 1) * other.total * (self.total - self.pass);
        let val2 = (self.total + 1) * self.total * (other.total - other.pass);
        val1.partial_cmp(&val2)
    }
}

impl Ord for ClassRatio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let val1 = (other.total + 1) * other.total * (self.total - self.pass);
        let val2 = (self.total + 1) * self.total * (other.total - other.pass);
        val1.cmp(&val2)
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = BinaryHeap::new();
        for c in &classes {
            heap.push(ClassRatio {
                pass: c[0] as i64,
                total: c[1] as i64,
            });
        }

        for _ in 0..extra_students {
            if let Some(mut class) = heap.pop() {
                class.pass += 1;
                class.total += 1;
                heap.push(class);
            }
        }

        let mut ans = 0.0;
        let count = classes.len() as f64;
        while let Some(class) = heap.pop() {
            ans += class.pass as f64 / class.total as f64;
        }
        ans / count
    }
}

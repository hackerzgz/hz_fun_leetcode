use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::max_total_fruits(vec![vec![2, 8], vec![6, 3], vec![8, 6]], 5, 4),
            9
        );
    }
}

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        0
    }
}

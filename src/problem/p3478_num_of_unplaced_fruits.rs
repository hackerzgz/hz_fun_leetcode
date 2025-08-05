pub struct Solution;

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]),
            1
        );

        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]),
            0
        );
    }

    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut buckets: Vec<i32>) -> i32 {}
}

struct SegTree {}

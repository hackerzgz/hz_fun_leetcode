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

        assert_eq!(
            Solution::max_total_fruits(
                vec![
                    vec![0, 9],
                    vec![4, 1],
                    vec![5, 7],
                    vec![6, 2],
                    vec![7, 4],
                    vec![10, 9]
                ],
                5,
                4
            ),
            14
        );

        assert_eq!(
            Solution::max_total_fruits(vec![vec![0, 3], vec![6, 4], vec![8, 5]], 3, 2),
            0
        );

        assert_eq!(
            Solution::max_total_fruits(vec![vec![0, 3], vec![2, 1], vec![5, 2]], 1, 4),
            4
        );
    }
}

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut max_fruits = 0i32;
        let mut current_sum = 0i32;

        let mut l = 0usize;
        for r in 0..fruits.len() {
            let pos = fruits[r][0];
            let fruit_count = fruits[r][1];
            current_sum += fruit_count;

            while l <= r
                && pos - fruits[l][0]
                    + (start_pos - fruits[l][0])
                        .abs()
                        .min((start_pos - pos).abs())
                    > k
            {
                current_sum -= fruits[l][1];
                l += 1;
            }

            max_fruits = max_fruits.max(current_sum);
        }

        max_fruits
    }
}

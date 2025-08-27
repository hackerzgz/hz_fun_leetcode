use crate::problem::Solver;

pub struct Solution;

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]),
            15
        );
        assert_eq!(
            Solution::count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            7
        );
    }
}

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    dp[i][j] = matrix[i][j];
                } else if matrix[i][j] == 0 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1])) + 1;
                }
                count += dp[i][j];
            }
        }

        count
    }
}

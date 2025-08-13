use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::soup_servings(50), 0.625);
        assert_eq!(Solution::soup_servings(100), 0.71875);
    }
}

impl Solution {
    fn soup_servings(n: i32) -> f64 {
        if n > 4450 {
            return 1f64;
        }

        let l = (n as usize + 24) / 25;
        let mut memo: Vec<Vec<f64>> = vec![vec![0_f64; l + 1]; l + 1];

        Self::dfs(&mut memo, l as i32, l as i32)
    }

    fn dfs(memo: &mut Vec<Vec<f64>>, i: i32, j: i32) -> f64 {
        if i <= 0 && j <= 0 {
            return 0.5;
        }
        if i <= 0 {
            return 1.0;
        }
        if j <= 0 {
            return 0.0;
        }

        let p = memo[i as usize][j as usize];
        if p > 0.0 {
            return p;
        }
        let ans = (Self::dfs(memo, i - 4, j)
            + Self::dfs(memo, i - 3, j - 1)
            + Self::dfs(memo, i - 2, j - 2)
            + Self::dfs(memo, i - 1, j - 3))
            * 0.25;
        memo[i as usize][j as usize] = ans;
        return ans;
    }
}

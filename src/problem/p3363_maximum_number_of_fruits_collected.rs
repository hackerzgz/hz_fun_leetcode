use crate::problem::Solver;

pub struct Solution;

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::max_collected_fruits(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 8, 7],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]),
            100
        );

        assert_eq!(
            Solution::max_collected_fruits(vec![vec![1, 1], vec![1, 1]]),
            4
        );
    }
}

impl Solution {
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        // 左上角只能沿着对角线走
        let mut ans = (0..n).map(|p| fruits[p][p]).sum::<i32>();

        // 先走右上角
        ans += dp(&fruits, n);
        // 镜像矩阵
        for i in 0..n {
            for j in 0..i {
                let tmp = fruits[i][j];
                fruits[i][j] = fruits[j][i];
                fruits[j][i] = tmp;
            }
        }
        // 在走左上角
        ans += dp(&fruits, n);

        ans
    }
}

fn dp(fruits: &Vec<Vec<i32>>, n: usize) -> i32 {
    let mut prev = vec![i32::MIN; n];
    let mut curr = vec![i32::MIN; n];

    prev[n - 1] = fruits[0][n - 1];
    for i in 1..n - 1 {
        for j in (n - 1 - i).max(i + 1)..n {
            let mut best = prev[j];
            if j > 0 {
                best = best.max(prev[j - 1]);
            }
            if j + 1 < n {
                best = best.max(prev[j + 1]);
            }
            curr[j] = best + fruits[i][j];
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n - 1]
}

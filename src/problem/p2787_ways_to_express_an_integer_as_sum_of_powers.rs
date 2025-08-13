use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::number_of_ways(10, 2), 1);
        assert_eq!(Solution::number_of_ways(4, 1), 2);
    }
}

// 0-1背包模板题
impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;
        let x = x as u32;

        // 生成所有可能的 k^x 候选数（k为正整数，k^x <= n）
        let mut candidates = Vec::new();
        let mut k: u64 = 1;
        loop {
            match k.checked_pow(x) {
                Some(val) if val <= n as u64 => {
                    candidates.push(val as usize);
                    k += 1;
                }
                _ => break, // 溢出或超过n时停止
            }
        }

        // 动态规划计算子集和方案数
        let mut dp = vec![0; n + 1];
        dp[0] = 1; // 初始状态：和为0有1种方案（空集）

        // 每个候选数只能使用一次（0-1背包问题）
        for &s in &candidates {
            // 从后往前遍历避免重复使用
            for j in (s..=n).rev() {
                dp[j] = (dp[j] + dp[j - s]) % MOD;
            }
        }

        dp[n]
    }

    pub fn number_of_ways_fast(n: i32, x: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0_i64; n + 1];
        f[0] = 1;
        for i in 1usize.. {
            let val = i.pow(x as u32);
            if val > n {
                break;
            }

            for s in (val..=n).rev() {
                f[s] += f[s - val];
            }
        }

        (f[n] % 1_000_000_007) as i32
    }
}

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
        assert_eq!(Solution::people_aware_of_secret(5, 2, 3), 2);
    }
}

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;
        let delay = delay as usize;
        let forget = forget as usize;

        let mut known = 0;
        let mut ans = 0;

        // diff 数组是 known 数组的差分数组。
        //
        // 对于初始值 known[1]=1，对应到差分数组上，就是 diff[1]=1 以及 diff[2]=−1。
        let mut diff = vec![0; n + 1];
        (diff[1], diff[2]) = (1, -1);

        for i in 1..=n {
            // 加上 diff[i] 后，known 表示恰好在第 i 天得知秘密的人数
            known = (known + diff[i]) % MOD;
            // 统计在第 n 天没有忘记秘密的人数
            if i >= n - forget + 1 {
                ans = (ans + known) % MOD;
            }

            // 恰好在第 i 天得知秘密的人，会在第 [i+delay, i+forget-1] 天分享秘密
            if i + delay <= n {
                diff[i + delay] = (diff[i + delay] + known) % MOD;
            }
            if i + forget <= n {
                // +MOD 保证结果非负
                diff[i + forget] = (diff[forget] - known + MOD) % MOD;
            }
        }

        ans
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }
    pub fn run(&self) {
        assert_eq!(Self::integer_replacement(8), 3);
        assert_eq!(Self::integer_replacement(7), 4);
        assert_eq!(Self::integer_replacement(4), 2);

        assert_eq!(Self::integer_replacement_greedy(8), 3);
        assert_eq!(Self::integer_replacement_greedy(7), 4);
        assert_eq!(Self::integer_replacement_greedy(4), 2);
    }

    /// input  |  output
    ///   8    |    3
    ///   7    |    4
    ///   4    |    2
    fn integer_replacement(n: i32) -> i32 {
        use std::collections::VecDeque;
        let mut fifo = VecDeque::with_capacity(1024);
        fifo.push_back((n as i64, 0));
        while let Some((t, times)) = fifo.pop_front() {
            if t == 1 {
                return times;
            }
            if t & 1 == 0 {
                fifo.push_back((t >> 1, times + 1));
            } else {
                fifo.push_back((t + 1, times + 1));
                fifo.push_back((t - 1, times + 1));
            }
        }
        -1
    }

    fn integer_replacement_greedy(n: i32) -> i32 {
        // 主要体现在对 xx 为奇数时的处理，我们总是处理 `x + 1` 和 `x - 1` 两种情况。
        //
        // 我们可以从二进制的角度进行分析：给定起始值 nn，求解将其变为 (000...0001) 的最小步数。
        //
        // 对于偶数（二进制最低位为 00）而言，我们只能进行一种操作，其作用是将当前值 xx 其进行一个单位的右移；
        // 对于奇数（二进制最低位为 11）而言，我们能够进行 +1 或 -1 操作，分析两种操作为 xx 产生的影响：
        //   - 对于 +1 操作而言：最低位必然为 1，此时如果次低位为 0 的话， +1 相当于将最低位和次低位交换；
        //     如果次低位为 1 的话，+1 操作将将「从最低位开始，连续一段的 1」进行消除（置零），并在连续一段的高一位添加一个 1；
        //   - 对于 -1 操作而言：最低位必然为 1，其作用是将最低位的 1 进行消除。
        // 因此，对于 xx 为奇数所能执行的两种操作，+1 能够消除连续一段的 1，只要次低位为 1（存在连续段），应当优先使用 +1 操作，但需要注意边界 x = 3 时的情况（此时选择 -1 操作）。
        let mut n = n;
        let mut ans = 0;
        while n > 1 {
            if n == 3 {
                return ans + 2;
            }
            ans += match n & 3 {
                0 => {
                    n >>= 2;
                    2
                }
                1 => {
                    n >>= 2;
                    3
                }
                2 => {
                    n >>= 1;
                    1
                }
                _ => {
                    n = (n >> 2) + 1;
                    3
                }
            };
        }
        ans
    }
}

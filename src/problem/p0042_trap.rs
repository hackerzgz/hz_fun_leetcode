use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }
    fn run(&self) {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}

impl Solution {
    // 单调栈（保证栈的单调性）
    //
    // 维护一个单调栈，单调栈存储的是下标，满足从栈底到栈顶的下标对应的数组 height 中的元素递减。
    // 从左到右遍历数组，遍历到下标 i 时，如果栈内至少有两个元素，记栈顶元素为 top，top 的下面一个元素是 left，则一定有 height[left]≥height[top]。如果 height[i]>height[top]，则得到一个可以接雨水的区域，该区域的宽度是 i−left−1，高度是 min(height[left],height[i])−height[top]，根据宽度和高度即可计算得到该区域能接的雨水量。
    // 为了得到 left，需要将 top 出栈。在对 top 计算能接的雨水量之后，left 变成新的 top，重复上述操作，直到栈变为空，或者栈顶下标对应的 height 中的元素大于或等于 height[i]。
    // 在对下标 i 处计算能接的雨水量之后，将 i 入栈，继续遍历后面的下标，计算能接的雨水量。遍历结束之后即可得到能接的雨水总量。
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut st: Vec<usize> = Vec::new();
        for (i, &h) in height.iter().enumerate() {
            // 判断栈顶元素的高度是否低于当前高度
            while !st.is_empty() && height[st[st.len() - 1]] <= h {
                let bottom_h = height[st.pop().unwrap()];
                if st.is_empty() {
                    break;
                }

                let left = st[st.len() - 1];
                let dh = height[left].min(h) - bottom_h;
                ans += dh * ((i - left - 1) as i32);
            }

            st.push(i);
        }
        ans
    }

    // 朴素的 DP
    pub fn trap_dp(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }

        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];
        left_max[0] = height[0];
        right_max[n - 1] = height[n - 1];
        for i in 1..n {
            left_max[i] = std::cmp::max(left_max[i - 1], height[i]);
            right_max[n - 1 - i] = std::cmp::max(right_max[n - 1 - i + 1], height[n - 1 - i])
        }

        let mut ans = 0i32;
        for i in 0..n {
            ans += std::cmp::min(left_max[i], right_max[i]) - height[i];
        }

        ans
    }
}

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }
    fn run(&self) {
        assert_eq!(
            Solution::product_queries_fast(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]]),
            vec![2, 4, 64]
        );

        assert_eq!(Solution::product_queries_fast(2, vec![vec![0, 0]]), vec![2]);
    }
}

// 其实用线段树也能做，但是因为实际二次幂的数组长度不大
impl Solution {
    pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers: Vec<i32> = Vec::with_capacity(16);
        while n > 0 {
            let x = n & -n;
            powers.push(x);
            n -= x;
        }

        const MODULO: i64 = 1_000_000_007;
        let mut ans: Vec<i32> = Vec::with_capacity(queries.len());
        for q in queries {
            let (lhs, rhs) = (q[0], q[1]);
            let mut r = 1_i64;
            for i in lhs..=rhs {
                r = r * powers[i as usize] as i64 % MODULO;
            }

            ans.push(r as i32);
        }

        ans
    }

    // 前缀和
    // https://leetcode.cn/problems/range-product-queries-of-powers/solutions/1895314/bao-li-yu-chu-li-by-endlesscheng-kt0t
    pub fn product_queries_fast(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MODULO: i32 = 1_000_000_007;
        let mut pow2 = Vec::with_capacity(436);
        pow2.push(1_i32);
        for i in 1..436 {
            pow2.push(pow2[i - 1] * 2 % MODULO);
        }

        let mut s = vec![0];
        while n > 0 {
            let e = n.trailing_zeros();
            s.push(s[s.len() - 1] + e);
            n = n & n - 1;
        }

        let mut ans: Vec<i32> = Vec::with_capacity(queries.len());
        for q in queries {
            let (lhs, rhs) = (q[0] as usize, q[1] as usize);
            let sum_e = s[rhs + 1] - s[lhs];
            ans.push(pow2[sum_e as usize]);
        }

        ans
    }
}

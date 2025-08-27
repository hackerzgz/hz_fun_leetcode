use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2), 10);
        assert_eq!(Solution::max_profit(vec![5, 4, 3], vec![1, 1, 0], 2), 9);
        assert_eq!(Solution::max_profit(vec![4, 7, 13], vec![-1, -1, 0], 2), 9);
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let plen = prices.len();

        let mut total = 0_i64;
        for i in 0..plen {
            total += prices[i] as i64 * strategy[i] as i64;
        }

        let origin = total;
        let mut ret = total;

        let k = k as usize;
        for i in 0..=plen - k {
            for j in i..i + (k / 2) {
                match strategy[j] {
                    -1 => total += prices[j] as i64,
                    1 => total -= prices[j] as i64,
                    _ => (),
                }
            }
            for j in i + (k / 2)..i + k {
                match strategy[j] {
                    -1 => total += prices[j] as i64 + prices[j] as i64,
                    0 => total += prices[j] as i64,
                    _ => (),
                }
            }

            // println!("ret: {}, total: {}", ret, total);
            ret = ret.max(total);
            total = origin;
        }
        ret
    }
}

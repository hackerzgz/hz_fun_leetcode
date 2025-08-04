pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(Self::min_cutting_cost(6, 5, 5), 5);
        assert_eq!(Self::min_cutting_cost(4, 4, 6), 0);
    }

    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let n = n as i64;
        let m = m as i64;
        let k = k as i64;
        if n > k {
            return (n - k) * k;
        }
        if m > k {
            return (m - k) * k;
        }
        0
    }
}

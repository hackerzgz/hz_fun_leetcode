pub struct Solution {}

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let (mut d, mut base) = (1, 1);
        while n > d * 9 * base {
            n -= d * 9 * base;
            d += 1;
            base *= 10;
        }
        n -= 1;
        ((base + n / d) / 10_i64.pow((d - 1 - n % d) as u32) % 10) as i32
    }
}

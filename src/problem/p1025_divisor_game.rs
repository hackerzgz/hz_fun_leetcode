pub struct Solution {}

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        if n & 1 == 0 {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1025() {
        assert_eq!(Solution::divisor_game(2), true);
        assert_eq!(Solution::divisor_game(3), false);
    }
}

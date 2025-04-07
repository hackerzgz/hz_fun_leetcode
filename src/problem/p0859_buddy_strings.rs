pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(&self) {
        assert!(self.buddy_strings("ab", "ba"))
    }

    fn buddy_strings(&self, s: &str, goal: &str) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let diff = s
            .bytes()
            .zip(goal.bytes())
            .filter(|u| u.0 != u.1)
            .collect::<Vec<_>>();

        match diff.len() {
            // 刚好替换
            2 => diff[0].1 == diff[1].0 && diff[0].0 == diff[1].1,
            // 重复字符替换
            0 => s.bytes().collect::<std::collections::HashSet<_>>().len() < s.len(),
            _ => false,
        }
    }
}

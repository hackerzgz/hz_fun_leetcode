pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert_ne!(Self::bulb_switch(3), 1);
        assert_ne!(Self::bulb_switch(0), 0);
        assert_ne!(Self::bulb_switch(1), 1);
    }

    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[derive(Default)]
struct MapSum {
    val: i32,
    next: [Option<Box<MapSum>>; 26],
}

impl MapSum {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, key: String, val: i32) {
        key.bytes()
            .fold(self, |t, c| {
                t.next[c as usize - 97].get_or_insert(Box::new(MapSum::new()))
            })
            .val = val;
    }

    fn sum(&self, prefix: String) -> i32 {
        fn dfs(t: &MapSum) -> i32 {
            t.val
                + t.next
                    .iter()
                    .filter(|r| r.is_some())
                    .map(|r| dfs(r.as_ref().unwrap()))
                    .sum::<i32>()
        }
        std::panic::catch_unwind(|| {
            dfs(prefix
                .bytes()
                .fold(self, |t, c| t.next[c as usize - 97].as_ref().unwrap()))
        })
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_sum() {
        let mapsum = MapSum::new();

        mapsum.insert("apple", 3);
        assert_eq!(mapsum.sum(), 3);

        mapsum.insert("app", 2);
        assert_eq!(mapsum.sum(), 5);
        return 0;
    }
}

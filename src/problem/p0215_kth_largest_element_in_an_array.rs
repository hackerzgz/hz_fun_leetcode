pub struct MaxHeap<T: std::cmp::Ord> {
    heap: Vec<T>,
    size: usize,
}

impl<T> MaxHeap<T>
where
    T: std::cmp::Ord,
{
    pub fn new(size: usize) -> Self {
        let heap = Vec::with_capacity(size);
        Self { heap, size: 0 }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn max(&self) -> Option<&T> {
        self.heap.get(0)
    }
    pub fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.shift_up(self.size as i32);
        self.size += 1;
    }
    pub fn pop_max(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            1 => {
                self.size = 0;
                self.heap.pop()
            }
            _ => {
                self.size -= 1;
                self.heap.swap(0, self.size);
                let result = self.heap.pop();
                self.shift_down(0); // reshape the heap
                result
            }
        }
    }

    fn shift_up(&mut self, mut index: i32) {
        let mut parent_index = (index - 1) / 2;
        let Self { heap, .. } = self;
        while parent_index >= 0 {
            if heap[parent_index as usize] < heap[index as usize] {
                heap.swap(parent_index as usize, index as usize);
                index = parent_index;
                parent_index = (index - 1) / 2;
            } else {
                break;
            }
        }
    }

    fn shift_down(&mut self, mut index: i32) {
        let Self { heap, .. } = self;
        let length = heap.len() as i32;
        let mut left_index = index * 2 + 1;
        let mut right_index = index * 2 + 2;
        while left_index < length {
            let mut swap_index = left_index;
            if right_index < length && heap[right_index as usize] > heap[left_index as usize] {
                swap_index = right_index;
            }
            if heap[swap_index as usize] > heap[index as usize] {
                heap.swap(swap_index as usize, index as usize);
                index = swap_index;
                left_index = index * 2 + 1;
                right_index = index * 2 + 2;
            } else {
                break;
            }
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(Self::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);

        assert_eq!(
            Self::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = MaxHeap::new(2 * k as usize);
        nums.iter().for_each(|v| heap.insert(v));
        for _ in 1..k {
            heap.pop_max();
        }
        *heap.pop_max().unwrap()
    }
}

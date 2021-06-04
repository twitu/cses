struct Heap<T>
where
    T: Copy + Ord,
{
    values: Vec<T>,
}

impl<T> Heap<T>
where
    T: Copy + Ord,
{
    fn new() -> Self {
        Heap { values: Vec::new() }
    }

    fn left_child(&self, n: usize) -> usize {
        n * 2 + 1
    }

    fn right_child(&self, n: usize) -> usize {
        n * 2 + 2
    }

    fn parent(&self, n: usize) -> usize {
        n - 1 / 2
    }

    fn last_index(&self) -> usize {
        self.values.len() - 1
    }

    fn heap_compare(&self, i: usize, j: usize) -> bool {
        let last_index = self.last_index();

        if i > last_index || j > last_index {
            return false;
        } else {
            self.values[i] > self.values[j]
        }
    }

    fn heap_swap(&mut self, i: usize, j: usize) {
        self.values.swap(i, j)
    }

    fn peek(&self) -> T {
        self.values[0]
    }

    fn add(&mut self, v: T) {
        self.values.push(v);
        self.bubble_up(self.last_index());
    }

    fn remove(&mut self, v: T) -> T {
        let last_index = self.values.len() - 1;
        let return_value = self.values[0];
        self.heap_swap(0, last_index);
        self.bubble_down(0);

        return_value
    }

    fn bubble_up(&mut self, index: usize) {
        let mut index = index;

        loop {
            if index == 0 {
                break;
            }

            let parent = self.parent(index);
            if self.heap_compare(index, parent) {
                self.heap_swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, index: usize) {
        let mut index = index;

        loop {
            let left = self.left_child(index);
            let right = self.right_child(index);

            if self.heap_compare(right, left) {
                if self.heap_compare(right, index) {
                    self.heap_swap(right, index);
                    index = right;
                } else {
                    break;
                }
            } else {
                if self.heap_compare(left, index) {
                    self.heap_swap(left, index);
                    index = left;
                } else {
                    break;
                }
            }
        }
    }
}


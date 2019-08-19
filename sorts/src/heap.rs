use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct PriorityQueue<T: PartialOrd + PartialEq + Clone + Default+Debug> {
    items: Vec<T>,
    length: usize,
}

impl<T: Eq + PartialOrd + PartialEq + Clone + Default +Debug> PriorityQueue<T> {
    #[allow(dead_code)]
    fn left_child_index(&self, n: usize) -> Option<usize> {
        let maybe = (n + 1) * 2 - 1;
        if maybe > self.length - 1 { None } else { Some(maybe) }
    }
    #[allow(dead_code)]
    fn right_child_index(&self, n: usize) -> Option<usize> {
        let maybe = (n + 1) * 2;
        if maybe > self.length - 1 { None } else { Some(maybe) }
    }
    #[allow(dead_code)]
    fn parent_index(&self, n: usize) -> Option<usize> {
        debug_assert!(n <= self.length);
        if n == 0 { None } else {
            Some((n + 1) / 2 - 1)
        }
    }
    #[allow(dead_code)]
    pub fn new()-> Self{
        PriorityQueue{
            items: vec![],
            length: 0
        }
    }
    #[allow(dead_code)]
    fn new_empty(length: usize, value: T) -> Self {
        let items = vec![value; length as usize];
        PriorityQueue {
            items,
            length,
        }
    }
    #[allow(dead_code)]
    fn insert(&self, value: T) -> Self {
        let mut result = self.clone();
        result.insert_mutable(value);
        result
    }
    #[allow(dead_code)]
    fn insert_mutable(&mut self, value: T) {
        self.items.push(value);
        self.length += 1;
        self.bubble_up(self.length - 1)
    }
    #[allow(dead_code)]
    fn bubble_up(&mut self, n: usize) {
        match self.parent_index(n) {
            None => (),
            Some(parent) => if &self.items[parent] < &self.items[n] {
                self.items.swap(parent, n);
                self.bubble_up(parent);
            }
        }
    }
    #[allow(dead_code)]
    fn bubble_down(&mut self, n: usize) {
        match self.right_child_index(n) {
            Some(right) => {
                let left = self.left_child_index(n).unwrap();
                if self.items[left] < self.items[right] {
                    if self.items[n] < self.items[right] {
                        self.items.swap(n, right);
                        self.bubble_down(right)
                    }
                } else {
                    if self.items[n] < self.items[left] {
                        self.items.swap(n, left);
                        self.bubble_down(left)
                    }
                }
            }
            None => {
                if let Some(left) = self.left_child_index(n) {
                    if self.items[left] > self.items[n] {
                        self.items.swap(n, left);
                        self.bubble_down(left);
                    }
                }
            }
        }
            }
    #[allow(dead_code)]
    fn delete_item_mutable(&mut self, n: usize) ->T {
        self.items.swap(n, self.length - 1);
        let result = self.items.pop().unwrap_or_else(|| {T::default()});
        self.length-=1;
        if self.length>0 {
            self.bubble_down(n);
        }
       result
    }
    #[allow(dead_code)]
    fn extract_head(&mut self) ->T{
        self.delete_item_mutable(0)
    }
    #[allow(dead_code)]
    fn get_length(&self) ->usize{
        self.length
    }
    #[allow(dead_code)]
    //сортирует не на месте (из-за этого лишняя память тратится), но идею я понял
    pub fn heapsort(array:Vec<T>)->Vec<T>{
        let mut pq = PriorityQueue::new();
        let mut init = array.clone();
        while init.len()>0{
            pq.insert_mutable(init.pop().unwrap());
        }
        let mut result =vec![];
        while pq.get_length() > 0 {
            result.push(pq.extract_head());
        }
        pq.items.iter().for_each(|x|println!("{:?}",x));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::PriorityQueue;

    #[test]
    fn find_index_test() {
        let pq = PriorityQueue::new_empty(31, 10);
        assert_eq!(pq.left_child_index(9), Some(19));
        assert_eq!(pq.right_child_index(12), Some(26));
        assert_eq!(pq.right_child_index(14), Some(30));
        assert_eq!(pq.left_child_index(15), None);
        assert_eq!(pq.parent_index(5), Some(2));
        assert_eq!(pq.parent_index(14), Some(6));
        assert_eq!(pq.parent_index(0), None);
    }

    #[test]
    fn insert_delete_test() {
        let mut pq = PriorityQueue::new_empty(1, 12);
        pq.insert_mutable(15);
        pq.insert_mutable(7);
        pq.insert_mutable(14);
        pq.insert_mutable(19);
        pq.insert_mutable(9);
        pq.insert_mutable(23);
        assert_eq!(pq.items, vec![23, 15, 19, 12, 14, 7, 9]);
        pq.delete_item_mutable(1);
        assert_eq!(pq.items, vec![23, 14, 19, 12, 9, 7]);
        pq.delete_item_mutable(0);
        assert_eq!(pq.items, vec![19, 14, 7, 12, 9]);

    }

    #[test]
    fn heapsort_test(){
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut init = (1..100).collect::<Vec<i32>>();
        init.shuffle(& mut rng);
        let result:Vec<i32> = PriorityQueue::heapsort(init);
        let mut sorted = (1..100).collect::<Vec<i32>>();
        sorted.sort_unstable_by(|a,b|b.cmp(a));
        assert_eq!(result,sorted);

    }
}
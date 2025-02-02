// The code is not passing yet.

use std::{cmp::Ordering, mem, ops::RangeFrom};

const MAX_KEY: usize = 100000;

pub struct LFUCache {
    capacity: usize,
    epoch_iter: RangeFrom<usize>,
    heap: Vec<Entry>,
    heap_index: Vec<Option<usize>>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;

        Self {
            capacity,
            heap: vec![],
            epoch_iter: 0..,
            heap_index: vec![None; MAX_KEY + 1],
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let key = key as usize;
        let index = match self.heap_index[key] {
            Some(index) => index,
            None => return -1,
        };

        let epoch = self.epoch_iter.next().unwrap();
        let mut entry = self.remove(index).unwrap();
        let value = entry.value;
        entry.weight.use_count += 1;
        entry.weight.epoch = epoch;
        self.insert(entry);

        self.check();

        value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        let key = key as usize;

        match self.heap_index[key] {
            Some(index) => {
                let epoch = self.epoch_iter.next().unwrap();
                let mut entry = self.remove(index).unwrap();
                entry.value = value;
                entry.weight.use_count += 1;
                entry.weight.epoch = epoch;
                self.insert(entry);
            }
            None => {
                if self.heap.len() == self.capacity {
                    self.remove(0);
                }

                let epoch = self.epoch_iter.next().unwrap();
                let entry = Entry {
                    key,
                    value,
                    weight: Weight {
                        use_count: 1,
                        epoch,
                    },
                };
                self.insert(entry);
            }
        }

        self.check();
    }

    fn insert(&mut self, entry: Entry) {
        let key = entry.key;
        let mut index = self.heap.len();

        self.heap.push(entry);
        self.heap_index[key] = Some(index);

        while index != 0 {
            let parent = to_parent_index(index);

            if self.heap[index] < self.heap[parent] {
                self.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn remove(&mut self, mut index: usize) -> Option<Entry> {
        let removed_entry = {
            if index >= self.heap.len() {
                return None;
            }
            let last_index = self.heap.len() - 1;

            if index == last_index {
                let entry = self.heap.pop().unwrap();
                self.heap_index[entry.key] = None;
                return Some(entry);
            } else {
                self.swap(index, last_index);
                let entry = self.heap.pop().unwrap();
                self.heap_index[entry.key] = None;
                entry
            }
        };

        while index < self.heap.len() {
            let ridx = to_right_index(index);

            match ridx.cmp(&self.heap.len()) {
                Ordering::Less => {
                    let lidx = ridx - 1;

                    if self.heap[lidx].weight < self.heap[index].weight {
                        if self.heap[lidx].weight < self.heap[ridx].weight {
                            self.swap(index, lidx);
                            index = lidx;
                        } else {
                            self.swap(index, ridx);
                            index = ridx;
                        }
                    } else if self.heap[ridx].weight < self.heap[index].weight {
                        self.swap(index, ridx);
                        index = ridx;
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    let lidx = ridx - 1;

                    if self.heap[lidx].weight < self.heap[index].weight {
                        self.swap(index, lidx);
                        index = lidx;
                    } else {
                        break;
                    }
                }
                Ordering::Greater => break,
            }
        }

        Some(removed_entry)
    }

    fn swap(&mut self, lidx: usize, ridx: usize) {
        if lidx == ridx {
            return;
        }

        // Swap heap node
        let (lnode, rnode) = get2(&mut self.heap, lidx, ridx).unwrap();
        mem::swap(lnode, rnode);

        // Swap heap index
        let lkey = lnode.key;
        let rkey = rnode.key;
        let (lpos, rpos) = get2(&mut self.heap_index, lkey, rkey).unwrap();

        let lpos = lpos.as_mut().unwrap();
        let rpos = rpos.as_mut().unwrap();
        mem::swap(lpos, rpos);
    }

    // fn print(&self) {
    //     dbg!(&self.heap);
    //     for (key, &index) in self.heap_index.iter().enumerate() {
    //         if let Some(index) = index {
    //             eprintln!("{key} -> {index}");
    //         }
    //     }
    // }

    fn check(&self) {
        let ok = self
            .heap_index
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(key, index)| Some((key, index?)))
            .all(|(key, index)| self.heap[index].key == key);
        assert!(ok);

        self.heap[1..].iter().zip(1..).all(|(node, index)| {
            let parent_index = to_parent_index(index);
            let parent = &self.heap[parent_index];
            node.weight >= parent.weight
        });
    }
}

fn to_parent_index(index: usize) -> usize {
    (index + 1) / 2 - 1
}

// fn to_left_index(index: usize) -> usize {
//     (index + 1) * 2 - 1
// }

fn to_right_index(index: usize) -> usize {
    (index + 1) * 2
}

fn get2<T>(slice: &mut [T], lidx: usize, ridx: usize) -> Option<(&mut T, &mut T)> {
    if lidx == ridx || lidx >= slice.len() || ridx >= slice.len() {
        return None;
    }

    unsafe {
        let ptr = slice.as_mut_ptr();
        let lptr = ptr.add(lidx);
        let rptr = ptr.add(ridx);
        let lref = lptr.as_mut().unwrap();
        let rref = rptr.as_mut().unwrap();
        Some((lref, rref))
    }
}

#[derive(Debug, Clone)]
struct Entry {
    pub key: usize,
    pub value: i32,
    pub weight: Weight,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

#[derive(Debug, Clone)]
struct Weight {
    pub use_count: usize,
    pub epoch: usize,
}

impl PartialEq for Weight {
    fn eq(&self, other: &Self) -> bool {
        self.use_count == other.use_count && self.epoch == other.epoch
    }
}

impl Eq for Weight {}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Weight {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.use_count.cmp(&other.use_count) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.epoch.cmp(&other.epoch)
    }
}

#[cfg(test)]
mod tests {
    use super::LFUCache;

    #[test]
    fn p460_test() {
        {
            let mut lfu = LFUCache::new(2);
            lfu.put(1, 1); // cache=[1,_], cnt(1)=1
            lfu.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
            assert_eq!(lfu.get(1), 1); // return 1
                                       // cache=[1,2], cnt(2)=1, cnt(1)=2
            lfu.put(3, 3); // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                           // cache=[3,1], cnt(3)=1, cnt(1)=2
            assert_eq!(lfu.get(2), -1); // return -1 (not found)
            assert_eq!(lfu.get(3), 3); // return 3
                                       // cache=[3,1], cnt(3)=2, cnt(1)=2
            lfu.put(4, 4); // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                           // cache=[4,3], cnt(4)=1, cnt(3)=2
            assert_eq!(lfu.get(1), -1); // return -1 (not found)
            assert_eq!(lfu.get(3), 3); // return 3
                                       // cache=[3,4], cnt(4)=1, cnt(3)=3
            assert_eq!(lfu.get(4), 4); // return 4
                                       // cache=[4,3], cnt(4)=2, cnt(3)=3    }
        }

        {
            let mut lfu = LFUCache::new(0);
            assert_eq!(lfu.get(1), -1);
            lfu.put(1, 10);
            assert_eq!(lfu.get(1), -1);
        }

        {
            let mut lfu = LFUCache::new(1);
            assert_eq!(lfu.get(1), -1);
            lfu.put(1, 10);
            assert_eq!(lfu.get(1), 10);
            lfu.put(1, 20);
            assert_eq!(lfu.get(1), 20);
            lfu.put(2, 30);
            assert_eq!(lfu.get(1), -1);
            assert_eq!(lfu.get(2), 30);
        }

        {
            let mut lfu = LFUCache::new(2);
            lfu.put(1, 10);
            lfu.put(2, 20);
            lfu.put(3, 30);
            assert_eq!(lfu.get(1), -1);
            assert_eq!(lfu.get(2), 20);
            assert_eq!(lfu.get(3), 30);
        }
    }
}

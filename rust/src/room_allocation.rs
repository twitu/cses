use std::collections::BinaryHeap;
use std::io::{stdin, Lines, Read};

struct Heap<T>
where
    T: Copy + Ord,
{
    size: usize,
    values: Vec<T>,
}

impl<T> Heap<T>
where
    T: Copy + Ord,
{
    fn new() -> Self {
        Heap {
            size: 0,
            values: Vec::new(),
        }
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

    fn heap_compare(&self, i: usize, j: usize) -> bool {
        let last_index = self.size - 1;

        if i > last_index || j > last_index {
            return false;
        } else {
            self.values[i] < self.values[j]
        }
    }

    fn heap_swap(&mut self, i: usize, j: usize) {
        self.values.swap(i, j)
    }

    fn peek(&self) -> T {
        self.values[0]
    }

    fn add(&mut self, v: T) {
        self.size += 1;
        self.values.push(v);
        self.bubble_up(self.size - 1);
    }

    fn remove(&mut self) -> T {
        self.size -= 1;
        let last_index = self.values.len() - 1;
        let return_value = self.values[0];
        self.heap_swap(0, last_index);
        self.bubble_down(0);

        return_value
    }

    fn remove_and_add(&mut self, value: T) {
        self.values[0] = value;
        self.bubble_down(0);
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Room {
    occupied_till: usize,
    id: usize,
}

// reverse ordering for occupied till date of room
// i.e room that will freed up sooner should come sooner
impl PartialOrd for Room {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(ordering) = self.occupied_till.partial_cmp(&other.occupied_till) {
            match ordering {
                std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Greater),
                std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Equal => return self.id.partial_cmp(&other.id),
            }
        } else {
            None
        }
    }
}

// reverse ordering for occupied till date of room
// i.e room that will freed up sooner should come sooner
impl Ord for Room {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.occupied_till.cmp(&other.occupied_till) {
            std::cmp::Ordering::Less => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => return self.id.cmp(&other.id),
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let _n_guests: usize = input.next().unwrap().parse().unwrap();
    let mut rooms: BinaryHeap<Room> = BinaryHeap::new();

    let mut guests: Vec<(usize, usize, usize)> = input
        .into_iter()
        .enumerate()
        .map(|(i, next_guest)| {
            let line: Vec<&str> = next_guest.split(' ').collect();
            let start: usize = line[0].parse().unwrap();
            let end: usize = line[1].parse().unwrap();

            (start, end, i)
        })
        .collect();

    // sort by arrival time
    guests.sort();

    // store rooms allocated to each guest
    let mut room_given: Vec<(usize, usize)> = Vec::new();

    for &(start, end, guest_id) in guests.iter() {
        if let (Some(room_to_be_free)) = rooms.peek() {
            // all rooms are occupied add new one
            if start <= room_to_be_free.occupied_till {
                let room_id = rooms.len() + 1;
                rooms.push(Room {
                    occupied_till: end,
                    id: room_id,
                });
                room_given.push((guest_id, room_id));

            // empty room is available reoccupy it
            } else {
                let mut free_room = rooms.pop().unwrap();
                free_room.occupied_till = end;
                rooms.push(free_room);
                room_given.push((guest_id, free_room.id));
            }
        } else {
            rooms.push(Room {
                occupied_till: end,
                id: 1,
            });
            room_given.push((guest_id, 1));
        }
    }

    println!("{}", rooms.len());
    room_given.sort();
    for (_, room_id) in room_given.iter() {
        print!("{} ", room_id);
    }
}

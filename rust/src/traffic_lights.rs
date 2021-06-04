use std::{cmp::Ordering, io::*};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    value: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: usize, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self { value, left, right }
    }

    fn insert(&mut self, new_value: Node) {
        match new_value.value.cmp(&self.value) {
            Ordering::Less => {
                match self.left {
                    Some(ref mut node) => node.insert(new_value),
                    None => self.left = Some(Box::new(new_value)),
                };
            }
            Ordering::Greater => {
                match self.right {
                    Some(ref mut node) => node.insert(new_value),
                    None => self.right = Some(Box::new(new_value)),
                };
            }
            _ => {}
        }
    }

    fn search(&self, value: usize) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                Some(ref node) => node.search(value),
                None => false,
            },

            Ordering::Greater => match self.right {
                Some(ref node) => node.search(value),
                None => false,
            },

            Ordering::Equal => true,
        }
    }

    fn traverse_and_then_mut<F>(&mut self, value: usize, f: F)
    where
        F: Fn(&mut Node) -> Node,
    {
        match value.cmp(&self.value) {
            Ordering::Less => {
                match self.left {
                    Some(ref mut node) => node.traverse_and_then_mut(value, f),
                    None => self.left = Some(Box::new(f(self))),
                };
            }
            Ordering::Greater => {
                match self.right {
                    Some(ref mut node) => node.traverse_and_then_mut(value, f),
                    None => self.right = Some(Box::new(f(self))),
                };
            }
            Ordering::Equal => {}
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();
    let first_line = input.next().unwrap();
    let second_line = input.next().unwrap();

    let first_inputs: Vec<&str> = first_line.split(' ').collect();
    let street_length: usize = first_inputs[0].parse().unwrap();
    let n_lights: usize = first_inputs[1].parse().unwrap();
}

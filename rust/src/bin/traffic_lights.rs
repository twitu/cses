use std::{cmp::Ordering, io::*};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    value: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: usize, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node { value, left, right }
    }
}

trait BinarySearchTree {
    fn insert(tree: &mut Option<Box<Node>>, value: usize);
    fn search(tree: &Option<Box<Node>>, value: usize) -> bool;
    fn search_boundary(
        tree: &Option<Box<Node>>,
        value: usize,
        prev_boundary: (usize, usize),
    ) -> (usize, usize);
    fn delete(tree: &mut Option<Box<Node>>, value: usize);
    fn find_max(tree: &Option<Box<Node>>) -> Option<usize>;
    fn pre_order_walk(tree: &Option<Box<Node>>);
}

impl BinarySearchTree for Node {
    fn insert(tree: &mut Option<Box<Node>>, value: usize) {
        let node = tree.take();

        let update_node = match node {
            Some(mut inner) => {
                match value.cmp(&inner.value) {
                    Ordering::Less => <Node as BinarySearchTree>::insert(&mut inner.left, value),
                    Ordering::Greater | Ordering::Equal => {
                        <Node as BinarySearchTree>::insert(&mut inner.right, value)
                    }
                }

                inner
            }
            None => Box::new(Node::new(value, None, None)),
        };

        tree.insert(update_node);
    }

    fn search(tree: &Option<Box<Node>>, value: usize) -> bool {
        if let Some(node) = tree {
            match value.cmp(&node.value) {
                Ordering::Equal => return false,
                Ordering::Less => return <Node as BinarySearchTree>::search(&node.left, value),
                Ordering::Greater => return <Node as BinarySearchTree>::search(&node.right, value),
            }
        } else {
            return false;
        }
    }

    fn search_boundary(
        tree: &Option<Box<Node>>,
        value: usize,
        prev_boundary: (usize, usize),
    ) -> (usize, usize) {
        if let Some(node) = tree {
            let (left_boundary, right_boundary) = prev_boundary;

            match value.cmp(&node.value) {
                Ordering::Less => <Node as BinarySearchTree>::search_boundary(
                    &node.left,
                    value,
                    (left_boundary, node.value - 1),
                ),
                Ordering::Greater => <Node as BinarySearchTree>::search_boundary(
                    &node.right,
                    value,
                    (node.value, right_boundary),
                ),
                Ordering::Equal => prev_boundary,
            }
        } else {
            prev_boundary
        }
    }

    fn delete(tree: &mut Option<Box<Node>>, value: usize) {
        let node = tree.take();

        if let Some(mut inner) = node {
            match value.cmp(&inner.value) {
                Ordering::Less => {
                    <Node as BinarySearchTree>::delete(&mut inner.left, value);
                    tree.insert(inner);
                }
                Ordering::Greater => {
                    <Node as BinarySearchTree>::delete(&mut inner.right, value);
                    tree.insert(inner);
                }
                Ordering::Equal => {
                    // is leaf node
                    match (inner.left, inner.right) {
                        // move right child to current node
                        (None, Some(right)) => {
                            tree.insert(right);
                        }
                        // move left child to current node
                        (Some(left), None) => {
                            tree.insert(left);
                        }
                        // leaf node: already delete because tree is None
                        (None, None) => {}
                        // handle both children
                        (Some(left), Some(right)) => {
                            let mut left_tree = Some(left);
                            let right_tree = Some(right);

                            // find and delete inorder successor value
                            let inorder_successor =
                                <Node as BinarySearchTree>::find_max(&left_tree).unwrap();
                            <Node as BinarySearchTree>::delete(&mut left_tree, inorder_successor);

                            // change self value to inorder successor
                            // and reconstruct inner node
                            inner.value = inorder_successor;
                            inner.left = left_tree;
                            inner.right = right_tree;
                            tree.insert(inner);
                        }
                    }
                }
            }
        };
    }

    fn find_max(tree: &Option<Box<Node>>) -> Option<usize> {
        if let Some(node) = tree {
            if node.right.is_none() {
                Some(node.value)
            } else {
                <Node as BinarySearchTree>::find_max(&node.right)
            }
        } else {
            None
        }
    }

    fn pre_order_walk(tree: &Option<Box<Node>>) {
        if let Some(node) = tree {
            print!("{} ", node.value);
            <Node as BinarySearchTree>::pre_order_walk(&node.left);
            <Node as BinarySearchTree>::pre_order_walk(&node.right);
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut input = input.lines();
    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .into_iter()
        .map(|val| val.parse().unwrap())
        .collect();
    let street_lights_pos: Vec<usize> = input
        .next()
        .unwrap()
        .split(' ')
        .into_iter()
        .map(|val| val.parse().unwrap())
        .collect();
    let street_length = first_line[0];
    let inital_unlit = (0, street_length - 1);

    let mut length_tree: Option<Box<Node>> = None;
    let mut light_tree: Option<Box<Node>> = None;
    <Node as BinarySearchTree>::insert(&mut length_tree, street_length);

    for new_light_pos in street_lights_pos.iter() {
        // get the range of unlit portion new light will split
        let (left_boundary, right_boundary) =
            <Node as BinarySearchTree>::search_boundary(&light_tree, *new_light_pos, inital_unlit);
        // add new light to current set of lights
        <Node as BinarySearchTree>::insert(&mut light_tree, *new_light_pos);

        let unlit_length = right_boundary - left_boundary + 1;
        let new_left_unlit_stretch = new_light_pos - left_boundary;
        let new_right_unlit_stretch = right_boundary - new_light_pos + 1;

        // delete the previous unlit stretch
        // add the two new unlit stretches created after the new light was added
        <Node as BinarySearchTree>::delete(&mut length_tree, unlit_length);
        <Node as BinarySearchTree>::insert(&mut length_tree, new_left_unlit_stretch);
        <Node as BinarySearchTree>::insert(&mut length_tree, new_right_unlit_stretch);

        // print current max unlit distance
        let max_unlit = <Node as BinarySearchTree>::find_max(&length_tree).unwrap();
        print!("{} ", max_unlit);
    }
}

extern crate num_traits;
extern crate rpds;


use std::collections::VecDeque;
use rpds::{Vector, Stack};

fn main() {
    for i in 0..15 {
        println!("{}", recur_fibo(i))
    }
    for i in 0..15 {
        println!("{}", lazy_fibo(i))
    }
}

#[allow(dead_code)]
fn new_position(list: Vector<i32>, new_head: i32) -> Vector<i32> {
    let without_last = list.drop_last().unwrap();
    let mut new_vector = rpds::Vector::new();
    new_vector.push_back_mut(new_head);
    without_last.iter().for_each(|x| new_vector.push_back_mut(*x));

    new_vector
}
#[allow(dead_code)]
fn lazy_fibo(n: i32) -> i32 {
    std::iter::successors(Some((0, 1)), |x| Some((x.1, x.0 + x.1)))
        .take((n + 1) as usize)
        .collect::<Vec<(i32, i32)>>()
        .last().unwrap().0
}

fn recur_fibo(x: i32) -> i32 {
    fn go(first: i32, second: i32, current: i32) -> i32 {
        if current == 0 { first } else {
            go(second, first + second, current - 1)
        }
    };
    go(0, 1, x)
}


type option_node<T: PartialOrd + Clone> = Option<Box<Node<T>>>;
#[allow(dead_code)]
struct Node<T> {
    pub value: T,
    left: option_node<T>,
    right: option_node<T>,
}
#[allow(dead_code)]
impl<T: PartialOrd + Clone> Node<T> {
    pub fn new(value: T) ->  option_node<T> {
        let root = Node {
            value: value,
            left: None,
            right: None,
        };
        Some(Box::new(root))
    }
}
#[allow(dead_code)]
impl<T: PartialOrd + Clone> BinarySearchTree<T> {
    pub fn new(value: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Node::new(value)
        }
    }

    pub fn append (&mut self, new_value: T) {
        let root = std::mem::replace(&mut self.root, None);
        self.root = self.append_helper(root, new_value);
    }

     fn append_helper (&mut self, tree: option_node<T>, new_value: T) -> option_node<T> {
        match tree {
            None => Node::new(new_value),
            Some(mut node) => {
                if new_value < node.value {
                    node.left = self.append_helper(node.left, new_value);
                    Some(node)
                } else {
                    node.right = self.append_helper(node.right, new_value);
                    Some(node)
                }
            }
        }
    }
}
#[allow(dead_code)]
pub struct BinarySearchTree<T> {
    root: option_node<T>
}
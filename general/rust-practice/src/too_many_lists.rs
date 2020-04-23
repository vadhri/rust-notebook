#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;
use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head: ListEnum<T>
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        println!("Drop list head..");
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ListEnum::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            value: elem,
            next:  mem::replace(&mut self.head, ListEnum::Empty),
        });
        self.head = ListEnum::Elem(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, ListEnum::Empty) {
            ListEnum::Empty => {
                None
            },
            ListEnum::Elem(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: ListEnum<T>
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) ->", self.value)
    }
}

#[derive(Debug)]
pub enum ListEnum<T> {
    Empty,
    Elem(Box<Node<T>>)
}

mod test {
    use super::List;
    use super::Node;
    use super::ListEnum;

    #[test]
    fn ll_take1_test_empty_list_pop() {
        let mut empty_list: List<f32> = List::new();
        assert_eq!(empty_list.pop(), None);
    }
    #[test]
    fn ll_take1_test_insert_pop_list() {
        let mut l: List<f32> = List::new();
        l.push(100.0);
        assert_eq!(l.pop(), Some(100.0));
    }
    #[test]
    fn ll_take1_too_many_lists_test() {
        let n1 = Box::new(Node { value: 1.0, next: ListEnum::Empty });
        let n2 = Box::new(Node { value: 2.0, next: ListEnum::Elem(n1) });

        let l = List {head :ListEnum::Elem(n2) };

        println!("{:?}", l);

        let mut empty_list: List<f32> = List::new();
        println!("{:?}", empty_list);

        empty_list.push(10.0);
        println!("{:?}", empty_list);
        empty_list.push(11.0);
        println!("{:?}", empty_list);
        println!("Removed element .. {:?}", empty_list.pop());
        println!("{:?}", empty_list);
    }
}

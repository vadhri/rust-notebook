#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;
use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head: ListEnum<T>
}

#[derive(Debug)]
pub struct IntoIter<T> (List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: ListEnum<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            value: elem,
            next:  self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => {
                None
            },
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
    pub fn peek(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            node.value
        })
    }

    pub fn peek_with_ref(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }
    pub fn peek_with_ref_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node)
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node)
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) ->", self.value)
    }
}

type ListEnum<T> = Option<Box<Node<T>>>;

mod test {
    use super::List;
    use super::Node;
    use super::ListEnum;

    #[test]
    fn ll_take2_test_empty_list_pop() {
        let mut empty_list: List<f32> = List::new();
        assert_eq!(empty_list.pop(), None);
    }
    #[test]
    fn ll_take2_test_insert_pop_list() {
        let mut l: List<f32> = List::new();
        l.push(100.0);
        assert_eq!(l.pop(), Some(100.0));
    }
    #[test]
    fn ll_take2_insert_peek_list() {
        let mut l: List<f32> = List::new();
        l.push(100.0);
        assert_eq!(l.peek(), Some(100.0));
        l.push(300.0);
        assert_eq!(l.peek(), Some(300.0));
    }
    #[test]
    fn ll_take2_insert_peek_ref_list() {
        let mut l: List<f32> = List::new();
        l.push(100.0);
        assert_eq!(l.peek_with_ref(), Some(&100.0));
        l.push(300.0);
        assert_eq!(l.peek(), Some(300.0));
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek_with_ref(), None);
        assert_eq!(list.peek_with_ref_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek_with_ref(), Some(&3));
        assert_eq!(list.peek_with_ref_mut(), Some(&mut 3));
        list.peek_with_ref_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek_with_ref(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn ll_take2_into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn ll_take2_test() {
        let mut list: List<f32> = List::new();
        list.push(10.0);
        list.push(11.0);
        list.pop();
        assert_eq!(list.peek_with_ref(), Some(&10.0));
        list.push(11.0);

        let opt = list.peek_with_ref_mut();
        opt.map(|value| {
            *value = 200.0;
        });

        assert_eq!(list.peek_with_ref(), Some(&200.0));
    }
    #[test]
    fn ll_take2_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn ll_take2_iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut(); 
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}

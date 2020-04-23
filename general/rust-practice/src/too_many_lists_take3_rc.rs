#![allow(dead_code)]
#![allow(unused_imports)]

use std::rc::Rc;
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

    pub fn push(&mut self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                value: elem,
                next: self.head.clone()
            }))
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn pop(&mut self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|value| value.next.clone())
        }
    }

    // pub fn peek(&mut self) -> Option<T> {
    //     let head = self.head.take().and_then(|node| std::rc::Rc::<too_many_lists_take3_rc::Node<T>>::try_unwrap())
    //
    // }

    pub fn peek_with_ref(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }
    // pub fn peek_with_ref_mut(&mut self) -> Option<&mut T> {
    //     self.head.as_mut().map(|node| {
    //         &mut node.value
    //     })
    // }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node)
        }
    }
    // pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    //     IterMut {
    //         next: self.head.as_mut().map(|node| &mut **node)
    //     }
    // }
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

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|node| {
//             self.next = node.next.as_mut().map(|node| &mut **node);
//             &mut node.value
//         })
//     }
// }

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.pop().head().clone().map(|a| *a)
//     }
// }

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                cur_link = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) ->", self.value)
    }
}

type ListEnum<T> = Option<Rc<Node<T>>>;

mod test {
    use super::List;
    use super::Node;
    use super::ListEnum;

    #[test]
    fn ll_take3_reference_count_basic_test() {
        let mut list = List::new();
        assert_eq!(list.head(), None);

        let mut list = list.push(1).push(2).push(3);
        assert_eq!(list.head(), Some(&3));

        let mut list = list.pop();
        assert_eq!(list.head(), Some(&2));

        let mut list = list.pop();
        assert_eq!(list.head(), Some(&1));

        let mut list = list.pop();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.pop();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn ll_take3_reference_count_iter() {
        let list = List::new().push(1).push(2).push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}

#![allow(warnings)]

use std::cell::RefCell;
use std::rc::Rc;

struct List<T> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: NodePtr<T>,
    prev: NodePtr<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head)
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.borrow_mut().next.take().map(|new_head| {
                new_head.borrow_mut().prev = None;
                new_head
            });
            if self.head.is_none() {
                self.tail = None;
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail)
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            self.tail = old_tail.borrow_mut().prev.take().map(|new_tail| {
                new_tail.borrow_mut().next = None;
                new_tail
            });
            if self.tail.is_none() {
                self.head = None;
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Populate list
        list.push_front(1);
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}

struct ListIntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> ListIntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter { list: self }
    }
}

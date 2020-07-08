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
            // WTF?!?
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
    }
}
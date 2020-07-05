#![allow(warnings)]
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct List<T> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: NodePtr<T>,
    prev: NodePtr<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        if self.head.is_none() && self.tail.is_none() {
            let new_node = Node {
                elem,
                next: None,
                prev: None,
            };
            let new_node = Rc::new(RefCell::new(new_node));
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(new_node);
        } else {
            // List.head (A)
            // List.tail (B) unchanged
            // New_node.next (C) = old head node
            // New_node.prev (D) = None
            // Old_head_node.next (E) unchanged
            // Old_head_node.prev (F) = new_head_node

            let old_head = self.head.take();
            let new_node = Node {
                elem,
                next: old_head, // (C)
                prev: None,     // (D)
            };
            let new_node = Rc::new(RefCell::new(new_node));
            new_node.borrow().next.as_ref().map(|old_head| {
                let mut old_head_node = old_head.borrow_mut();
                old_head_node.prev = Some(new_node.clone()); // (F)
            });

            self.head = Some(new_node); // (A)
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        None
    }

    fn push_back(&mut self, elem: T) {}

    fn pop_back(&mut self) -> Option<T> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn mine() {
        let mut list = List::new();

        // Populate list
        list.push_front(1);
        list.push_front(2);

        //dbg!(&list);
    }

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

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}

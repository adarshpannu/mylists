#![allow(warnings)]

#[derive(Debug)]
struct List<T> {
    head: NodePtr<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let mut new_node = Node { elem, next: None };
        self.head
            .take()
            .map(|prev_head| new_node.next = Some(prev_head));
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        let prev_head = self.head.take();
        prev_head.map(|prev_node| {
            self.head = prev_node.next;
            prev_node.elem
        })
    }
}

type NodePtr<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: NodePtr<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_push_pop() {
        let mut lst: List<i32> = List::new();
        let elems = vec![-10, 27, 5, 2, 0, 11];
        for &elem in elems.iter() {
            lst.push(elem);
        }

        for &elem in elems.iter().rev() {
            assert_eq!(lst.pop(), Some(elem));
        }
        assert_eq!(lst.pop(), None);
        assert_eq!(lst.pop(), None);
    }
}

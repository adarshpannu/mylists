#![allow(warnings)]

#[derive(Debug)]
struct List<T> {
    head: NodePtr<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let mut new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|prev_node| {
            self.head = prev_node.next;
            prev_node.elem
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|noderef| &noderef.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|noderef| &mut noderef.elem)
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
    fn basic_push_pop_peek() {
        let mut lst: List<i32> = List::new();
        let mut elems = vec![-10, 27, 5, 2, 0, 11];

        for &elem in elems.iter() {
            lst.push(elem);
        }

        let peek_elem = lst.peek_mut().unwrap();
        assert_eq!(*peek_elem, 11);
        *peek_elem = 111;

        elems.pop();
        elems.push(111);

        for &elem in elems.iter().rev() {
            assert_eq!(lst.peek(), Some(&elem));
            assert_eq!(lst.pop(), Some(elem));
        }

        assert_eq!(lst.pop(), None);
        assert_eq!(lst.pop(), None);
        assert_eq!(lst.peek(), None);
    }

    #[test]
    fn iter_test() {
        let mut lst: List<i32> = List::new();
        let mut elems = vec![-10, 27, 5, 2, 0, 11];

        for &elem in elems.iter() {
            lst.push(elem);
        }

        for elem in lst {
            println!("{}", elem);
        }
    }
}

struct ListIntoIter<T>(List<T>);

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iterators
impl<T> List<T> {
    fn into_iter(self) -> ListIntoIter<T> {
        ListIntoIter(self)
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
    }
}

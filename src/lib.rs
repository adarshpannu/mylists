#![allow(warnings)]
#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct List<T> {
    head: NodePtr<T>,
}

type NodePtr<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: NodePtr<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let new_node = match std::mem::replace(&mut self.head, None) {
            None => Node { elem, next: None },
            Some(prev_head) => Node {
                elem,
                next: Some(prev_head),
            },
        };
        self.head = Some(Box::new(new_node))
    }

    fn pop(&mut self) -> Option<T> {
        return match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(prev_head) => {
                self.head = prev_head.next;
                Some(prev_head.elem)
            }
        };
    }

    fn get(&self, ix: usize) -> Option<&T> {
        let mut node_opt = &self.head;
        let mut i = 0;
        loop {
            match node_opt {
                None => return None,
                Some(node) => {
                    if i == ix {
                        return Some(&node.elem);
                    } else {
                        node_opt = &node.next;
                        i += 1;
                    }
                }
            }
        }
    }
}

struct ListIntoIter<T>(List<T>);
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
    }
}

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn push_pop() {
        use super::{List, Node, Point};

        let mut lst: List<Point> = List::new();

        lst.push(Point(-1, -2));
        lst.push(Point(0, 0));
        lst.push(Point(11, 22));
        lst.push(Point(111, 222));

        assert_eq!(lst.get(0).unwrap().0, 111);
        assert_eq!(lst.get(1).unwrap().0, 11);
        assert_eq!(lst.get(2).unwrap().0, 0);
        assert_eq!(lst.get(3).unwrap().0, -1);
        assert!(lst.get(4).is_none());
        assert!(lst.get(400).is_none());

        let mut lst: List<i32> = List::new();

        lst.push(-1);
        lst.push(0);
        lst.push(11);
        assert_eq!(lst.pop().unwrap(), 11);
        assert_eq!(lst.pop().unwrap(), 0);
        assert_eq!(lst.pop().unwrap(), -1);
    }

    #[test]
    fn test_into_iter() {
        use super::{List, Node, Point};

        let mut lst: List<i32> = List::new();

        lst.push(-1);
        lst.push(0);
        lst.push(11);

        let mut iter = lst.into_iter();
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(-1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_for_loop() {
        use super::{List, Node, Point};

        let mut lst: List<i32> = List::new();

        lst.push(-1);
        lst.push(0);
        lst.push(11);

        for elem in lst {
            println!("{:?}", elem)
        }
    }

    fn test_iter() {
        use super::{List, Node, Point};

        let mut lst: List<i32> = List::new();

        lst.push(-1);
        lst.push(0);
        lst.push(11);

        for elem in &lst {
            println!("{:?}", elem)
        }
    }

}

#![allow(warnings)]

use std::rc::Rc;

#[derive(Debug)]
struct List<T> {
    head: NodePtr<T>,
}

type NodePtr<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: NodePtr<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn append(&self, elem: T) -> List<T> {
        let new_node = Node {
            elem,
            //next: self.head.as_ref().map(|node| Rc::clone(&node)), <- My code
            next: self.head.clone(), // <- Code from book, lol
        };
        List {
            head: Some(Rc::new(new_node)),
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    fn tail_orig(&self) -> List<T> {
        if self.head.is_none() {
            // Empty list
            return List { head: None };
        }
        let head_node = self.head.as_ref().unwrap();
        if head_node.next.is_none() {
            // List with one element
            return List { head: None };
        } else {
            // List with two or more elements
            let new_head_node = head_node.next.as_ref().unwrap();
            return List {
                head: Some(Rc::clone(new_head_node)),
            };
        }
    }

    fn tail(&self) -> List<T> {
        if let Some(cur_head) = self.head.as_ref() {
            if let Some(new_head) = cur_head.next.as_ref() {
                return List {
                    head: Some(Rc::clone(new_head)),
                };
            }
        }
        // List with zero or one elements
        return List { head: None };
    }

    pub fn tail_from_book(&self) {
        let head = self.head.as_ref().and_then(|node| node.next.clone());
    }
    
}

#[derive(Debug, Eq, PartialEq)]
struct Point(i32, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let mut lst1 = List::new();

        let lst1 = lst1
            .append(Point(10, 10))
            .append(Point(20, 20))
            .append(Point(30, 30));

        let lst2 = lst1.tail();

        assert_eq!(lst1.head(), Some(&Point(30, 30)));
        assert_eq!(lst2.head(), Some(&Point(20, 20)));
    }
}

struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    fn iter(&self) -> ListIter<T> {
        ListIter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test_from_book {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}

#![allow(warnings)]

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

/********** IntoIterator **********/
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

/********** Iterator **********/
struct ListIter<'a, T> {
    cur: &'a NodePtr<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            None => None,
            Some(node) => {
                let retval = &(**node).elem;
                self.cur = &(**node).next;
                Some(retval)
            }
        }
    }
}

impl<T> List<T> {
    fn iter(&self) -> ListIter<T> {
        ListIter { cur: &self.head }
    }
}

/********** Iterator Alternate **********/
struct ListIterAlt<'a, T> {
    curnode: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIterAlt<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curnode.map(|node| {
            self.curnode = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T> List<T> {
    fn iter_alt(&self) -> ListIterAlt<T> {
        let curnode = self.head.as_ref().map(|head| &**head);
        ListIterAlt { curnode }
    }
}

/********** Iterator Mut **********/

struct ListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let nodeptr = self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        });
        nodeptr
    }
}

impl<T> List<T> {
    fn iter_mut(&mut self) -> ListIterMut<T> {
        let nodeptr = self.head.as_mut().map(|node| &mut **node);
        ListIterMut { next: nodeptr }
    }
}

#[derive(Debug)]
struct Point(i32, i32);

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
    fn into_iter_test() {
        let mut lst: List<i32> = List::new();
        let elems = vec![-10, 27, 5, 2, 0, 11];

        for &elem in elems.iter() {
            lst.push(elem);
        }

        for elem in lst {
            println!("{}", elem);
        }
    }

    #[test]
    fn iter_test() {
        let mut lst: List<Point> = List::new();
        let elems = vec![-10, 27, 5, 2, 0, 11];

        for &elem in elems.iter() {
            lst.push(Point(elem, -elem));
        }

        for elem in lst.iter_alt() {
            println!("{:?}", *elem);
        }
    }
    #[test]
    fn iter_mut_test() {
        let mut lst: List<Point> = List::new();
        let elems = vec![-10, 27, 5, 2, 0, 11];

        for &elem in elems.iter() {
            lst.push(Point(elem, elem * 2));
        }

        for elem in lst.iter_mut() {
            elem.0 = -elem.0;
            elem.1 = -elem.1;
        }

        for elem in lst.iter_alt() {
            println!("{:?}", *elem);
        }
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}

mod test_from_book {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
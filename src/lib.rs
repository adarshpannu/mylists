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
        let new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &(*node).elem)
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut prev_node| {
            self.head = prev_node.next.take();
            prev_node.elem
        })
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        println!("Drop List");
    }

    /*
    let cur = self.head;
    while (cur != null) {
        cur_next = cur->next;
        delete cur;
        cur = cur_next;
    }
    */
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Drop Node");
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

struct ListIter<'a, T> {
    cur: &'a NodePtr<T>,
}

impl<T> List<T> {
    fn iter(&self) -> ListIter<T> {
        ListIter { cur: &self.head }
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.as_ref().map(|node| {
            let retval = &(*node).elem;
            self.cur = &(*node).next;
            retval
        })
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

        assert_eq!(lst.peek(), Some(&11));

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

        let mut ix = 0;
        for elem in lst {
            match ix {
                0 => assert_eq!(elem, 11),
                1 => assert_eq!(elem, 0),
                2 => assert_eq!(elem, -1),
                _ => panic!("wtf"),
            }
            ix += 1;
        }
    }

    #[test]
    fn test_iter() {
        use super::{List, Node, Point};

        let mut lst: List<i32> = List::new();

        lst.push(-1);
        lst.push(0);
        lst.push(11);

        for elem in lst.iter().enumerate() {
            match elem.0 {
                0 => assert_eq!(elem.1, &11),
                1 => assert_eq!(elem.1, &0),
                2 => assert_eq!(elem.1, &-1),
                _ => panic!("wtf"),
            }
        }
    }

    #[test]
    fn test_mut_iter() {
        use super::{List, Node, Point};
        let mut lst: List<i32> = List::new();
        lst.push(-1);
        for elem in lst.mut_iter() {
            *elem = *elem + 10;
        }
    }
}

/*
struct List<T> {
    head: NodePtr<T>,
}

type NodePtr<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: NodePtr<T>,
}
*/

struct ListMutIter<'a, T> {
    cur: &'a NodePtr<T>,
}

impl<T> List<T> {
    fn mut_iter(&mut self) -> ListMutIter<T> {
        ListMutIter { cur: &self.head }
    }
}

impl<'a, T> Iterator for ListMutIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        /*
        match self.cur {
            None => None,
            Some(mut node) => {
                let mut retval = &mut *node;
                //self.cur = &*node.next;
                Some(&mut retval.elem)
            }
        }
        */
        None
    }
}

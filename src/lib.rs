#![allow(warnings)]

struct List<T> {
    head: NodePtr<T>,
}

type NodePtr<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: NodePtr<T>
}

#[test]
fn test() {

}

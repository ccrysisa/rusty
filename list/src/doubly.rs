use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
            prev: None,
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let node = Rc::new(RefCell::new(Node::new(elem)));
        match self.head.take() {
            Some(head) => {
                node.borrow_mut().next = Some(head.clone());
                head.borrow_mut().prev = Some(node.clone());
                self.head = Some(node.clone());
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            match node.borrow_mut().next.take() {
                Some(next) => {
                    next.borrow_mut().prev.take();
                    self.head = Some(next.clone());
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_mut_front(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn push_back(&mut self, elem: T) {
        let node = Rc::new(RefCell::new(Node::new(elem)));
        match self.tail.take() {
            Some(tail) => {
                node.borrow_mut().prev = Some(tail.clone());
                tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node.clone());
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|node| {
            match node.borrow_mut().prev.take() {
                Some(prev) => {
                    prev.borrow_mut().next.take();
                    self.tail = Some(prev.clone());
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_mut_back(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIterator::into_iter(self)
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new(); // []
        assert_eq!(list.pop_front(), None);

        list.push_front(1); // [1]
        list.push_front(2); // [2, 1]
        list.push_front(3); // [3, 2, 1]

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));

        list.push_front(4); // [4]
        list.push_front(5); // [5, 4]

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn basics2() {
        let mut list = List::new(); // []
        assert_eq!(list.pop_back(), None);

        list.push_back(1); // [1]
        list.push_back(2); // [1, 2]
        list.push_back(3); // [1, 2, 3]

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), None);

        list.push_front(4); // [4]
        list.push_back(5); // [4, 5]

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new(); // []
        assert!(list.peek_front().is_none());

        list.push_front(1); // [1]
        list.push_front(2); // [2, 1]
        list.push_front(3); // [3, 2, 1]

        assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(*list.peek_front().unwrap(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(*list.peek_front().unwrap(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.peek_front().is_none());
    }

    #[test]
    fn peek2() {
        let mut list = List::new(); // []
        assert!(list.peek_front().is_none());

        list.push_front(1); // [1]
        list.push_back(2); // [1, 2]
        list.push_front(3); // [3, 1, 2]

        assert_eq!(*list.peek_back().unwrap(), 2);
        *list.peek_mut_back().unwrap() = 100; // [3, 1, 100]
        assert_eq!(*list.peek_back().unwrap(), 100);

        assert_eq!(list.pop_front(), Some(3)); // [1, 100]
        assert_eq!(*list.peek_front().unwrap(), 1);
        *list.peek_mut_front().unwrap() = 200; // [200, 100]
        assert_eq!(*list.peek_mut_front().unwrap(), 200);

        assert_eq!(list.pop_back(), Some(100)); // [200]
        assert_eq!(*list.peek_back().unwrap(), 200);
        assert_eq!(list.pop_back(), Some(200)); // []
        assert!(list.peek_front().is_none());
    }

    #[test]
    fn iter() {
        let mut list = List::new(); // []
        list.push_front(1); // [1]
        list.push_back(2); // [1, 2]
        list.push_front(3); // [3, 1, 2]

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), None);

        let mut list = List::new(); // []
        list.push_back("hello".to_string()); // ["hello"]
        list.push_back("world".to_string()); // ["hello", "world"]
        list.push_back("!".to_string()); // ["hello", "world", "!"]

        let expect = [
            String::from("hello"),
            String::from("world"),
            String::from("!"),
        ];
        let mut i = 0;
        for x in list {
            assert_eq!(x, expect[i]);
            i += 1;
        }
    }
}

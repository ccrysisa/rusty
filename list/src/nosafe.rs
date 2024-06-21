use std::ptr;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut node = Box::new(Node { elem, next: None });
        let raw_tail: *mut _ = &mut *node;
        if self.tail.is_null() {
            self.head = Some(node);
        } else {
            unsafe {
                (*self.tail).next = Some(node);
            }
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let next = head.next;
            if next.is_none() {
                self.tail = ptr::null_mut();
            }
            self.head = next;
            head.elem
        })
    }

    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.into_iter()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
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
        self.0.pop()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.head.as_deref())
    }
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.head.as_deref_mut())
    }
}

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new(); // []
        assert_eq!(list.pop(), None);

        list.push(1); // [1]
        list.push(2); // [1, 2]
        list.push(3); // [1, 2, 3]

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));

        list.push(4); // [4]
        list.push(5); // [4, 5]

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new(); // []
        list.push(1); // [1]
        list.push(2); // [1, 2]
        list.push(3); // [1, 2, 3]

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut list = List::new(); // []
        list.push("hello".to_string()); // ["hello"]
        list.push("world".to_string()); // ["hello", "world"]
        list.push("!".to_string()); // ["hello", "world", "!"]

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

    #[test]
    fn iter() {
        let mut list = List::new(); // []
        list.push(1); // [1]
        list.push(2); // [1, 2]
        list.push(3); // [1, 2, 3]

        // let mut iter = (&list).into_iter();
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        let mut list = List::new(); // []
        list.push("hello".to_string()); // ["hello"]
        list.push("world".to_string()); // ["hello", "world"]
        list.push("!".to_string()); // ["hello", "world", "!"]

        let expect = [
            String::from("hello"),
            String::from("world"),
            String::from("!"),
        ];
        let mut i = 0;
        for x in &list {
            assert_eq!(x, &expect[i]);
            i += 1;
        }
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new(); // []
        list.push(1); // [1]
        list.push(2); // [1, 2]
        list.push(3); // [1, 2, 3]

        // let mut iter = (&mut list).into_iter();
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

        let mut expect = [1, 2, 3];
        let mut i = 0;
        for x in &mut list {
            assert_eq!(x, &expect[i]);
            *x = expect.len() - i;
            i += 1;
        }

        expect.reverse();
        i = 0;
        for x in &list {
            assert_eq!(x, &expect[i]);
            i += 1;
        }
    }
}

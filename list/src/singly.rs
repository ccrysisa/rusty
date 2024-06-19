type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let next = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(next);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
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
        // Iter(self.head.as_ref().map(|node| node.as_ref()))
        Iter(self.head.as_deref())
    }
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            // self.0 = node.next.as_ref().map(|next| next.as_ref());
            self.0 = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
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

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.into_iter()
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
        list.push(2); // [2, 1]
        list.push(3); // [3, 2, 1]

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        list.push(4); // [4]
        list.push(5); // [5, 4]

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn generic() {
        let mut list = List::new(); // []
        assert_eq!(list.pop(), None);

        list.push(1); // [1]
        list.push(2); // [2, 1]
        list.push(3); // [3, 2, 1]
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        let mut list = List::new(); // []
        assert_eq!(list.pop(), None);

        list.push("hello".to_string()); // ["hello"]
        list.push("world".to_string()); // ["world", "hello"]
        list.push("!".to_string()); // ["!", "world", "hello"]
        assert_eq!(list.pop(), Some(String::from("!")));
        assert_eq!(list.pop(), Some(String::from("world")));
        assert_eq!(list.pop(), Some(String::from("hello")));
    }

    #[test]
    fn peek() {
        let mut list = List::new(); // []
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1); // [1]
        list.push(2); // [2, 1]
        list.push(3); // [3, 2, 1]

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|x| *x = 100);
        assert_eq!(list.peek(), Some(&100));
        assert_eq!(list.peek_mut(), Some(&mut 100));

        assert_eq!(list.pop(), Some(100));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new(); // []
        list.push(1); // [1]
        list.push(2); // [2, 1]
        list.push(3); // [3, 2, 1]

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        let mut list = List::new(); // []
        list.push("hello".to_string()); // ["hello"]
        list.push("world".to_string()); // ["world", "hello"]
        list.push("!".to_string()); // ["!", "world", "hello"]

        let expect = [
            String::from("!"),
            String::from("world"),
            String::from("hello"),
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
        list.push(2); // [2, 1]
        list.push(3); // [3, 2, 1]

        // let mut iter = (&list).into_iter();
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        let mut list = List::new(); // []
        list.push("hello".to_string()); // ["hello"]
        list.push("world".to_string()); // ["world", "hello"]
        list.push("!".to_string()); // ["!", "world", "hello"]

        let expect = [
            String::from("!"),
            String::from("world"),
            String::from("hello"),
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
        list.push(2); // [2, 1]
        list.push(3); // [3, 2, 1]

        // let mut iter = (&mut list).into_iter();
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);

        let mut expect = [3, 2, 1];
        let mut i = 0;
        for x in &mut list {
            assert_eq!(x, &expect[i]);
            i += 1;
            *x = i;
        }

        expect.reverse();
        i = 0;
        for x in &list {
            assert_eq!(x, &expect[i]);
            i += 1;
        }
    }
}

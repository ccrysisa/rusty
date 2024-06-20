use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

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

    // It takes a list and an element, and returns a List.
    pub fn prepend(&mut self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    // It takes a list and returns the whole list with the first element removed.
    pub fn tail(&mut self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    // returns a reference to the first element.
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
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

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(node) = link {
            if let Ok(ref mut node) = Rc::try_unwrap(node) {
                link = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.head(), None);

        // let mut list = list.prepend(1); // [1]
        // let mut list = list.prepend(2); // [2, 1]
        // let mut list = list.prepend(3); // [3, 2, 1]
        let mut list = list.prepend(1).prepend(2).prepend(3); // [3, 2, 1]
        assert_eq!(list.head(), Some(&3));

        let mut list = list.tail(); // [2, 1]
        assert_eq!(list.head(), Some(&2));

        let list = list.tail().tail(); // []
        assert_eq!(list.head(), None);

        let mut list = List::new();
        assert_eq!(list.head(), None);

        let mut list = list
            .prepend("hello".to_string())
            .prepend("world".to_string()); // ["world", "hello"]
        assert_eq!(list.head(), Some(&String::from("world")));

        let mut list = list.tail(); // ["hello"]
        assert_eq!(list.head(), Some(&String::from("hello")));

        let list = list.tail(); // []
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new(); // []
        let list = list.prepend(1).prepend(2).prepend(3); // [3, 2, 1]

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        let mut list = List::new(); // []
        let list = list.prepend("hello").prepend("world").prepend("!"); // ["!", "world", "hello"]

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
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node { elem, next: None });
        self.push_node(node);
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| node.elem)
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.peek_node().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.peek_mut_node().map(|node| &mut node.elem)
    }

    fn peek_node(&self) -> Option<&Node<T>> {
        self.head.as_deref()
    }

    fn peek_mut_node(&mut self) -> Option<&mut Node<T>> {
        self.head.as_deref_mut()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

pub struct Deque<T> {
    left: Stack<T>,
    right: Stack<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self {
            left: Stack::new(),
            right: Stack::new(),
        }
    }

    pub fn push_left(&mut self, elem: T) {
        self.left.push(elem)
    }
    pub fn push_right(&mut self, elem: T) {
        self.right.push(elem)
    }
    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }
    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }
    pub fn peek_left(&self) -> Option<&T> {
        self.left.peek()
    }
    pub fn peek_right(&self) -> Option<&T> {
        self.right.peek()
    }
    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
    }
    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }

    pub fn go_left(&mut self) -> bool {
        self.left
            .pop_node()
            .map(|node| self.right.push_node(node))
            .is_some()
    }

    pub fn go_right(&mut self) -> bool {
        self.right
            .pop_node()
            .map(|node| self.left.push_node(node))
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::Deque;
    use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new(); // []
        assert_eq!(stack.pop(), None);

        stack.push(1); // [1]
        stack.push(2); // [2, 1]
        stack.push(3); // [3, 2, 1]

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        stack.push(4); // [4]
        stack.push(5); // [5, 4]

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new(); // []
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);

        stack.push(1); // [1]
        stack.push(2); // [2, 1]
        stack.push(3); // [3, 2, 1]

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        stack.peek_mut().map(|x| *x = 100);
        assert_eq!(stack.peek(), Some(&100));
        assert_eq!(stack.peek_mut(), Some(&mut 100));

        assert_eq!(stack.pop(), Some(100));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn walk_aboot() {
        let mut deque = Deque::new(); // [_]

        deque.push_left(0); // [0,_]
        deque.push_right(1); // [0, _, 1]
        assert_eq!(deque.peek_left(), Some(&0));
        assert_eq!(deque.peek_right(), Some(&1));

        deque.push_left(2); // [0, 2, _, 1]
        deque.push_left(3); // [0, 2, 3, _, 1]
        deque.push_right(4); // [0, 2, 3, _, 4, 1]

        while deque.go_left() {} // [_, 0, 2, 3, 4, 1]

        assert_eq!(deque.pop_left(), None);
        assert_eq!(deque.pop_right(), Some(0)); // [_, 2, 3, 4, 1]
        assert_eq!(deque.pop_right(), Some(2)); // [_, 3, 4, 1]

        deque.push_left(5); // [5, _, 3, 4, 1]
        assert_eq!(deque.pop_right(), Some(3)); // [5, _, 4, 1]
        assert_eq!(deque.pop_left(), Some(5)); // [_, 4, 1]
        assert_eq!(deque.pop_right(), Some(4)); // [_, 1]
        assert_eq!(deque.pop_right(), Some(1)); // [_]

        assert_eq!(deque.pop_right(), None);
        assert_eq!(deque.pop_left(), None);
    }
}

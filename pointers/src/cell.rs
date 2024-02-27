use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating ant references, because we nerver give ant out
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn spawn_array() {
        unsafe impl<T> Sync for Cell<T> {}

        use std::sync::Arc;
        let x = Arc::new(Cell::new(0));
        let x1 = Arc::clone(&x);
        let count = 1024 * 1024;
        let th1 = std::thread::spawn(move || {
            for _ in 0..count {
                let old = x1.get();
                x1.set(old + 1);
            }
        });
        let x2 = Arc::clone(&x);
        let th2 = std::thread::spawn(move || {
            for _ in 0..count {
                let old = x2.get();
                x2.set(old + 1);
            }
        });
        th1.join().unwrap();
        th2.join().unwrap();
        assert_ne!(x.get(), 2 * count);
    }
}

// test spawn
/// ```compile_fail
/// use std::sync::Arc;
/// let x = Arc::new(Cell::new(42));
/// let x1 = Arc::clone(&x);
/// std::thread::spawn(move || {
///     x1.set(43);
/// });
/// let x2 = Arc::clone(&x);
/// std::thread::spawn(move || {
///     x2.set(44);
/// });
/// ```

// test_copy
/// ```compile_fail
/// use cell::Cell;
/// let x = Cell::new(String::from("hello"));
/// let first = x.get();
/// ```

#[allow(dead_code)]
struct CompileFailTest;

#![allow(unused)]

//----------------------------------------------------------------------------//
// Delclarative Macro
//----------------------------------------------------------------------------//
macro_rules! delmac {
    () => {};

    ($arg1:ty => $arg2:ident) => {
        type $arg2 = $arg1;
    };

    ($x:ident) => {
        $x += 1;
    };
}

delmac!();
delmac![];
delmac! {}

delmac! {
    u32 => Alsou32
}

#[test]
fn foo() {
    let mut x: Alsou32 = 42;
    delmac!(x);
    assert_eq!(x, 43);
}

//----------------------------------------------------------------------------//
// Trait MaxValue
//----------------------------------------------------------------------------//

trait MaxValue {
    fn max_value() -> Self;
}

#[allow(unused)]
macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

#[test]
fn max_test() {
    max_impl!(u32);
    max_impl!(i32);
    max_impl!(u64);
    max_impl!(i64);

    assert_eq!(u32::max_value(), u32::MAX);
    assert_eq!(i32::max_value(), i32::MAX);
    assert_eq!(u64::max_value(), u64::MAX);
    assert_eq!(i64::max_value(), i64::MAX);
}

//----------------------------------------------------------------------------//
// Vector Macros
//----------------------------------------------------------------------------//

#[macro_export]
macro_rules! avec {
    // () => {
    //     Vec::new()
    // };

    // ($elem:expr) => {{
    //     let mut v = Vec::new();
    //     v.push($elem);
    //     v
    // }};

    // ($elem1:expr, $elem2:expr) => {{
    //     let mut v = Vec::new();
    //     v.push($elem1);
    //     v.push($elem2);
    //     v
    // }};

    // ($($elem:expr),+) => {{
    //     let mut v = Vec::new();
    //     $(v.push($elem);)+
    //     v
    // }};

    ($($elem:expr),*) => {{
        // chek that count is const since const must be known in compile time
        const C: usize = $crate::count![@COUNT; $($elem),*];

        #[allow(unused_mut)]
        let mut v = Vec::with_capacity(C);
        $(v.push($elem);)*
        v
    }};

    ($($elem:expr,)*) => {
        $crate::avec![$($elem),*]
    };

    ($elem:expr; $count:expr) => {{
        // let count = $count;
        // let mut v = Vec::with_capacity(count);
        // v.extend(std::iter::repeat($elem).take(count));
        let mut v = Vec::new();
        v.resize($count, $elem);
        v
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($elem:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $elem]),*])
    };

    (@SUBST; $_elem: expr) => {
        ()
    }
}

#[test]
fn empty_vec() {
    let v: Vec<u32> = avec!();
    assert!(v.is_empty());
}

#[test]
fn single() {
    let v: Vec<u32> = avec![42];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 42);
}

#[test]
fn double() {
    let v: Vec<u32> = avec![42, 43];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 43);
}

#[test]
fn tailing() {
    let v: Vec<u32> = avec![42, 43,];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 43);
}

#[test]
fn clone_2() {
    let v: Vec<u32> = avec![42; 2];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 42);
}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(42);
    let v: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 42);
}

/// ```compile_fail
/// let v: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

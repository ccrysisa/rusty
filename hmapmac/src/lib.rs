#![allow(unused)]

use std::collections::HashMap;

//----------------------------------------------------------------------------//
// HashMap Macros
//----------------------------------------------------------------------------//

#[macro_export]
macro_rules! ahmap {
    // () => {
    //     ::std::collections::HashMap::new()
    // };

    // ($key:expr=>$value:expr) => {{
    //     let mut hmap = ::std::collections::HashMap::new();
    //     hmap.insert($key, $value);
    //     hmap
    // }};

    // ($($key:expr=>$value:expr),+ $(,)?) => {{
    //     let mut hmap = ::std::collections::HashMap::new();
    //     $(hmap.insert($key, $value);)*
    //     hmap
    // }};

    ($($key:expr=>$value:expr),*) => {{
        // chek that count is const since const must be known in compile time
        const C: usize = $crate::count![@COUNT; $($key),*];

        #[allow(unused_mut)]
        let mut hmap = ::std::collections::HashMap::with_capacity(C);
        $(hmap.insert($key, $value);)*
        hmap
    }};

    ($($key:expr=>$value:expr,)*) => {{
        $crate::ahmap![$($key=>$value),*]
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
fn empty_hashmap() {
    let hmap: HashMap<u32, &str> = ahmap!();
    assert!(hmap.is_empty());
}

#[test]
fn single() {
    let hmap: HashMap<u32, &str> = ahmap![42=>"abc"];
    assert!(!hmap.is_empty());
    assert_eq!(hmap.len(), 1);
    assert_eq!(hmap.get(&42), Some(&"abc"));
    assert_eq!(hmap.get(&47), None);
}

#[test]
fn double() {
    let hmap: HashMap<u32, &str> = ahmap![42=>"abc", 43=>"joh"];
    assert!(!hmap.is_empty());
    assert_eq!(hmap.len(), 2);
    assert_eq!(hmap.get(&42), Some(&"abc"));
    assert_eq!(hmap.get(&43), Some(&"joh"));
    assert_eq!(hmap.get(&47), None);
}

#[test]
fn tailing() {
    let hmap: HashMap<u32, &str> = ahmap![42=>"abc", 43=>"joh",];
    assert!(!hmap.is_empty());
    assert_eq!(hmap.len(), 2);
    assert_eq!(hmap.get(&42), Some(&"abc"));
    assert_eq!(hmap.get(&43), Some(&"joh"));
    assert_eq!(hmap.get(&47), None);
}

/// ```compile_fail
/// let hmap: HashMap<u32, &str> = hmapmac::ahmap![42: "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

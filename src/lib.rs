/*!
`flat_vec` provides a macro to flatten nested `Vec`s.
Particularly useful when you want to write a rules of [`egg`](https://github.com/egraphs-good/egg) which contains rules both `=>` and `<=>`.
*/

/// `flat_vec!` just like `vec!` but can be used to flatten nested `vec!`s.
/// Order of elements is preserved.
///
/// ```rust
/// use flat_vec::flat_vec;
///
/// assert_eq!(flat_vec!(1), vec![1]);
/// assert_eq!(flat_vec!(1, 2), vec![1, 2]);
/// assert_eq!(flat_vec!(1, 2, 3), vec![1, 2, 3]);
/// assert_eq!(flat_vec!(flat vec![1], 2), vec![1, 2]);
/// assert_eq!(flat_vec!(1, flat vec![2]), vec![1, 2]);
/// ```
#[macro_export]
macro_rules! flat_vec {
    () => {
        ::std::vec::Vec::new()
    };
    (flat $elem:expr $(,)?) => {
        $elem
    };
    ($elem:expr $(,)?) => {
        ::std::vec![$elem]
    };
    (
        flat $elem:expr ,
        $($rest:tt)+
    )  => {{
        let mut t = $elem;
        ::std::vec::Vec::extend(&mut t, flat_vec!($($rest)+));
        t
    }};
    (
        $elem:expr ,
        $($rest:tt)+
     ) => {{
        let mut t = vec![$elem];
        ::std::vec::Vec::extend(&mut t, flat_vec!($($rest)+));
        t
    }};
}

#[test]
fn test_flat_vec() {
    let v: Vec<i32> = flat_vec![];
    assert_eq!(v, vec![]);
    assert_eq!(flat_vec!(1), vec![1]);
    assert_eq!(flat_vec!(1, 2), vec![1, 2]);
    assert_eq!(flat_vec!(1, 2, 3), vec![1, 2, 3]);
    assert_eq!(flat_vec!(flat vec![1], 2), vec![1, 2]);
    assert_eq!(flat_vec!(1, flat vec![2]), vec![1, 2]);
}

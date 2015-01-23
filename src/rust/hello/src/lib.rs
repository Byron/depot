

/// Adds one to the input integer and returns the result
///
/// ```rust{.example}
/// assert!(mylib::add_one(1) == 2);
/// ```
/// And another one
///
/// ```test_harness{.should_fail}
/// #[test]
/// fn foo() {
///     panic!("oops! (will run & register as a failed test)")
/// }
/// ```
pub fn add_one(v: i32) -> i32 {
    //! **Note:** This documentation should appear in the parent doc
    v + 1
}

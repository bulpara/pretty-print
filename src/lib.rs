/// A macro to simplify pretty-printing of debug information using `println!`.
///
/// This macro prints the provided value using Rust's pretty print formatting (`{:#?}`),
/// which is particularly useful for printing complex structures in a more readable form.
///
/// # Examples
///
/// ```
/// use pretty_print::pretty_print;
///
/// let my_data = vec![1, 2, 3];
/// pretty_print!(my_data);
/// ```
///
/// This will output:
/// ```text
/// [
///     1,
///     2,
///     3,
/// ]
/// `
#[macro_export]
macro_rules! pretty_print {
    ($val: expr) => {
        println!("{:#?}", $val);
    };
}

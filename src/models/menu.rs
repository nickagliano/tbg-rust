// TODO: Move menu logic out of lib.rs
struct Menu<'a, T: std::fmt::Display> {
    message: &'a str,
    options: Vec<T>,
    selected_index: usize,
    use_simulate_typing: bool,
}

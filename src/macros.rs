#[macro_export]
macro_rules! arc_mutex {
    ($expr:expr) => {
        Arc::new(Mutex::new($expr))
    };
}

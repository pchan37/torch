pub fn debug_println(message: &str) {
    if cfg!(debug_assertions) {
        println!("{}", message);
    }
}

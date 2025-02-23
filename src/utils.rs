use dirs::home_dir;

// Change the alias to `Box<error::Error>`.
pub type Expect<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Shorten a path as string
pub fn short_path(path: &str) -> String {
    let binding = home_dir().unwrap();
    let home = binding.to_str().unwrap();
    if path.starts_with(home) {
        path.replace(home, "~")
    } else {
        path.to_string()
    }
}

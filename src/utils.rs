// Change the alias to `Box<error::Error>`.
pub type Expect<T> = std::result::Result<T, Box<dyn std::error::Error>>;

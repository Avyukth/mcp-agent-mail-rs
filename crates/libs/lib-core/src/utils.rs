use slug;

// Placeholder for `slugify` if it's not needed directly from `utils.rs`
pub fn slugify(text: &str) -> String {
    slug::slugify(text)
}

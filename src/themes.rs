pub const DEFAULT_THEME: &str = include_str!("themes/default.css");
pub const DARK_THEME: &str = include_str!("themes/dark.css");
pub const LIGHT_THEME: &str = include_str!("themes/light.css");

pub fn get_theme(name: &str) -> &'static str {
    match name {
        "dark" => DARK_THEME,
        "light" => LIGHT_THEME,
        _ => DEFAULT_THEME,
    }
}
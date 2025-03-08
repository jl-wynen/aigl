use crate::color::Color;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Theme {
    pub app_background: Color,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::Read;

    #[test]
    fn deserialize_theme() {
        let mut file = std::fs::File::open(
            std::path::PathBuf::from(env!("CARGO_WORKSPACE_DIR"))
                .join("resources")
                .join("themes")
                .join("radix.ron"),
        )
        .unwrap();
        let mut ron_string = String::new();
        file.read_to_string(&mut ron_string).unwrap();
        let theme: Theme = ron::from_str(&ron_string).unwrap();
        assert_eq!(theme.app_background, Color::from_hex("#ff0000").unwrap());
    }
}

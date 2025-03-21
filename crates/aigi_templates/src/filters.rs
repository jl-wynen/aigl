use minijinja::{Error, ErrorKind};

pub(crate) fn to_identifier(s: &str, convention: &str) -> Result<String, Error> {
    match convention {
        "PascalCase" => Ok(to_identifier_pascal_case(s)),
        "snake_case" => Ok(to_identifier_snake_case(s)),
        _ => Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid naming convention: {convention}"),
        )),
    }
}

fn to_identifier_pascal_case(s: &str) -> String {
    s.split([' ', '\t', '\n', '-', '_'])
        .map(capitalize_word)
        .collect()
}

fn to_identifier_snake_case(s: &str) -> String {
    s.replace([' ', '\t', '\n', '-'], "_")
    // let words: Vec<_> = s.split([' ', '\t', '\n', '-']).collect();
    // words.join("_")
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().chain(chars).collect(),
        None => word.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn to_identifier_works_with_pascal_case() -> Result<(), Error> {
        let c = "PascalCase";
        assert_eq!(to_identifier("UnChanGed", c)?, "UnChanGed");
        assert_eq!(to_identifier("capitalized", c)?, "Capitalized");
        assert_eq!(to_identifier("With space", c)?, "WithSpace");
        assert_eq!(
            to_identifier("With  several    spaces", c)?,
            "WithSeveralSpaces"
        );
        assert_eq!(to_identifier("with-dash", c)?, "WithDash");
        assert_eq!(
            to_identifier("-mixed -dash  and -- space ", c)?,
            "MixedDashAndSpace"
        );
        Ok(())
    }

    #[test]
    fn to_identifier_works_with_snake_case() -> Result<(), Error> {
        let c = "snake_case";
        assert_eq!(to_identifier("unChan_ged", c)?, "unChan_ged");
        assert_eq!(to_identifier("With space", c)?, "With_space");
        assert_eq!(
            to_identifier("With  several    spaces", c)?,
            "With__several____spaces"
        );
        assert_eq!(to_identifier("with-dash", c)?, "with_dash");
        assert_eq!(
            to_identifier("-mixed -dash  and -- space ", c)?,
            "_mixed__dash__and____space_"
        );
        Ok(())
    }
}

//! Validated resource name. Raw input in any case -> PascalCase internally.

use crate::error::CliError;
use heck::ToUpperCamelCase;

/// A validated, normalised resource name.
///
/// Construct via [`ResourceName::parse`]. The only accessor is
/// [`ResourceName::pascal`], which yields the PascalCase form substituted for
/// the `$NAME$` token in templates.
#[derive(Debug, Clone)]
pub struct ResourceName {
    pascal: String,
}

impl ResourceName {
    /// Validate then transform to PascalCase.
    ///
    /// Accepts alphanumeric characters and the separators `-`, `_`, ` `.
    /// Rejects empty input, control characters, path separators (`/`, `\`),
    /// `.` (which would allow traversal like `../x`), and any input whose
    /// normalised form is empty.
    pub fn parse(raw: &str) -> Result<Self, CliError> {
        // Reject control characters on the raw input before trimming, so a
        // trailing `\t` is caught rather than silently stripped.
        if raw.chars().any(|c| c.is_control()) {
            return Err(CliError::InvalidName(raw.to_string()));
        }

        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(CliError::InvalidName(raw.to_string()));
        }

        for ch in trimmed.chars() {
            let allowed = ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | ' ');
            if !allowed {
                return Err(CliError::InvalidName(raw.to_string()));
            }
        }

        let pascal = trimmed.to_upper_camel_case();
        if pascal.is_empty() {
            return Err(CliError::InvalidName(raw.to_string()));
        }

        Ok(Self { pascal })
    }

    /// The PascalCase form, e.g. `"UserProfile"`.
    pub fn pascal(&self) -> &str {
        &self.pascal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_separators_to_pascal() {
        for raw in ["user-profile", "userProfile", "user_profile", "user profile"] {
            assert_eq!(ResourceName::parse(raw).unwrap().pascal(), "UserProfile");
        }
    }

    #[test]
    fn single_word_upper_and_lower() {
        assert_eq!(ResourceName::parse("user").unwrap().pascal(), "User");
        assert_eq!(ResourceName::parse("USER").unwrap().pascal(), "User");
        assert_eq!(ResourceName::parse("User").unwrap().pascal(), "User");
    }

    #[test]
    fn rejects_empty() {
        assert!(ResourceName::parse("").is_err());
        assert!(ResourceName::parse("   ").is_err());
    }

    #[test]
    fn rejects_path_traversal_and_separators() {
        assert!(ResourceName::parse("../x").is_err());
        assert!(ResourceName::parse("a/b").is_err());
        assert!(ResourceName::parse("a\\b").is_err());
        assert!(ResourceName::parse("a.b").is_err());
    }

    #[test]
    fn rejects_control_chars() {
        assert!(ResourceName::parse("user\nprofile").is_err());
        assert!(ResourceName::parse("user\t").is_err());
    }
}

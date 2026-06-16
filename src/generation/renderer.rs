//! Template substitution. The only token is `$NAME$` -> PascalCase. Behind a
//! trait so a future `$name$` / `$NAME_KEBAB$` token is an extension.

use crate::domain::resource_name::ResourceName;

pub trait TemplateRenderer {
    fn render(&self, template: &str, name: &ResourceName) -> String;
}

/// Replaces every `$NAME$` with the PascalCase resource name.
pub struct NamePlaceholderRenderer;

impl TemplateRenderer for NamePlaceholderRenderer {
    fn render(&self, template: &str, name: &ResourceName) -> String {
        template.replace("$NAME$", name.pascal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitutes_pascal_name() {
        let name = ResourceName::parse("user-profile").unwrap();
        let out = NamePlaceholderRenderer.render("class $NAME$Handler", &name);
        assert_eq!(out, "class UserProfileHandler");
    }

    #[test]
    fn leaves_non_token_text_untouched() {
        let name = ResourceName::parse("user").unwrap();
        let out = NamePlaceholderRenderer.render("no token here", &name);
        assert_eq!(out, "no token here");
    }

    #[test]
    fn replaces_every_occurrence() {
        let name = ResourceName::parse("user").unwrap();
        let out = NamePlaceholderRenderer.render("$NAME$ and $NAME$", &name);
        assert_eq!(out, "User and User");
    }
}

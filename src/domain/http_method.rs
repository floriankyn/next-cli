//! The HTTP methods this CLI knows how to scaffold.
//!
//! Adding a method = a new variant here + two template files + the match arms
//! below. Nothing in `generation` or `io` changes (Open/Closed).

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    /// Uppercase form, e.g. `"GET"`.
    pub fn upper(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }

    /// Whether the request carries a body (affects the controller fragment).
    pub fn has_body(&self) -> bool {
        matches!(self, HttpMethod::Post | HttpMethod::Put)
    }

    /// The embedded handler template (`GET.ts` content) for this method.
    pub fn handler_template(&self) -> &'static str {
        match self {
            HttpMethod::Get => include_str!("../templates/handler_get.ts.tmpl"),
            HttpMethod::Post => include_str!("../templates/handler_post.ts.tmpl"),
            HttpMethod::Put => include_str!("../templates/handler_put.ts.tmpl"),
            HttpMethod::Delete => include_str!("../templates/handler_delete.ts.tmpl"),
        }
    }

    /// The embedded controller fragment for this method (appended after the
    /// shared preamble).
    pub fn controller_fragment(&self) -> &'static str {
        match self {
            HttpMethod::Get => include_str!("../templates/controller_get.ts.tmpl"),
            HttpMethod::Post => include_str!("../templates/controller_post.ts.tmpl"),
            HttpMethod::Put => include_str!("../templates/controller_put.ts.tmpl"),
            HttpMethod::Delete => include_str!("../templates/controller_delete.ts.tmpl"),
        }
    }

    /// The `route.ts` import line template for this method.
    pub fn route_import(&self) -> &'static str {
        match self {
            HttpMethod::Get => include_str!("../templates/route_import_get.ts.tmpl"),
            HttpMethod::Post => include_str!("../templates/route_import_post.ts.tmpl"),
            HttpMethod::Put => include_str!("../templates/route_import_put.ts.tmpl"),
            HttpMethod::Delete => include_str!("../templates/route_import_delete.ts.tmpl"),
        }
    }

    /// The `route.ts` export line template for this method.
    pub fn route_export(&self) -> &'static str {
        match self {
            HttpMethod::Get => include_str!("../templates/route_export_get.ts.tmpl"),
            HttpMethod::Post => include_str!("../templates/route_export_post.ts.tmpl"),
            HttpMethod::Put => include_str!("../templates/route_export_put.ts.tmpl"),
            HttpMethod::Delete => include_str!("../templates/route_export_delete.ts.tmpl"),
        }
    }

    /// All known methods, in canonical order.
    pub fn all() -> [HttpMethod; 4] {
        [
            HttpMethod::Get,
            HttpMethod::Post,
            HttpMethod::Put,
            HttpMethod::Delete,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_strings() {
        assert_eq!(HttpMethod::Get.upper(), "GET");
        assert_eq!(HttpMethod::Post.upper(), "POST");
        assert_eq!(HttpMethod::Put.upper(), "PUT");
        assert_eq!(HttpMethod::Delete.upper(), "DELETE");
    }

    #[test]
    fn body_only_for_post_and_put() {
        assert!(!HttpMethod::Get.has_body());
        assert!(HttpMethod::Post.has_body());
        assert!(HttpMethod::Put.has_body());
        assert!(!HttpMethod::Delete.has_body());
    }

    #[test]
    fn all_is_sorted_and_complete() {
        assert_eq!(
            HttpMethod::all(),
            [
                HttpMethod::Get,
                HttpMethod::Post,
                HttpMethod::Put,
                HttpMethod::Delete
            ]
        );
    }
}

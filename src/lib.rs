mod utils;

use wasm_bindgen::prelude::*;
use inflector::cases::titlecase::to_title_case;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static REPLACEMENTS: &'static [&'static [&str]] = &[
    &["-", " "], &["_", " "],
    &["[", " "], &["]", " "],
    &["id", "ID"], &["Id", "ID"],
    &["Url", "URL"], &["Uri", "URI"],
];

#[wasm_bindgen]
pub fn superhumanize(s: &str) -> String {
    let prepped = String::from(to_title_case(&s));
    let result = REPLACEMENTS
        .iter()
        .fold(prepped, |acc, pair| acc.replace(pair[0], pair[1]));
    println!("result: {:?}", result);
    return String::from(result.trim());
}

#[cfg(test)]
mod superhumanize_tests {
    fn super_helper(s: &str) -> String { 
        return super::superhumanize(s);
    }

    #[test]
    fn replaces_underscore_with_space() {
        assert_eq!(super_helper("with_underscore"), "With Underscore");
    }

    #[test]
    fn replaces_dash_with_space() {
        assert_eq!(super_helper("with-dash"), "With Dash");
    }

    #[test]
    fn replaces_braces_with_space() {
        assert_eq!(super_helper("with[braces]"), "With Braces");
    }

    #[test]
    fn handles_special_tokens() {
        assert_eq!(super_helper("with id"), "With ID");
        assert_eq!(super_helper("with Id"), "With ID");
        assert_eq!(super_helper("with Url"), "With URL");
        assert_eq!(super_helper("with Uri"), "With URI");
    }

    #[test]
    fn handles_camel_case() {
        assert_eq!(super_helper("tomSmith"), "Tom Smith");
    }
}
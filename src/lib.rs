//! A Rust macro utility for conditionally constructing strings, primarily used for CSS class names.
//! Inspired by the JavaScript [clsx](https://github.com/lukeed/clsx) package.
//!
//! # Examples
//!
//! ```rust
//! use clsx_r::clsx;
//!
//! // Basic usage
//! assert_eq!(clsx!("foo"), "foo");
//! assert_eq!(clsx!("foo", "bar"), "foo bar");
//!
//! // With conditions
//! let is_active = true;
//! assert_eq!(clsx!("foo" => is_active), "foo");
//!
//! // Mixed usage
//! let dynamic_class = "dynamic";
//! assert_eq!(
//!     clsx!("base", dynamic_class, "active" => true, "disabled" => false),
//!     "base dynamic active"
//! );
//! ```
#[macro_export]
macro_rules! clsx {
    // Case: No args
    () => {
        String::new()
    };

    // Case: trailing comma after args
    (@internal $classes:ident ,) => {};

    // Case: empty argument after a trailing comma
    (@internal $classes:ident) => {};

    // Case: single argument without condition
    ($class:expr) => {
        $class.to_string()
    };

    // Case: conditional argument with remaining token
    (@internal $classes:ident $class:expr => $cond:expr, $($rest:tt)*) => {{
        if $cond {
            $classes.push($class.to_string());
        }
        $crate::clsx!(@internal $classes $($rest)*);
    }};

    // Case: last conditional argument
    (@internal $classes:ident $class:expr => $cond:expr) => {{
        if $cond {
            $classes.push($class.to_string());
        }
    }};

    // Case: non-conditional argument with remaining tokens
    (@internal $classes:ident $class:expr, $($rest:tt)*) => {{
        $classes.push($class.to_string());
        $crate::clsx!(@internal $classes $($rest)*);
    }};

    // Case: last non-conditional argument
    (@internal $classes:ident $class:expr) => {{
        $classes.push($class.to_string());
    }};

    // Entry point for handling arguments
    ($($args:tt)*) => {{
        let mut classes = Vec::new();
        $crate::clsx!(@internal classes $($args)*);
        classes.into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_usage() {
        assert_eq!(clsx!("class1"), "class1");
        assert_eq!(clsx!("class1", "class2"), "class1 class2");
        assert_eq!(clsx!("class1", "", "class2"), "class1 class2");
    }

    #[test]
    fn test_conditional_usage() {
        assert_eq!(clsx!("class1" => true, "class2" => false), "class1");
        assert_eq!(clsx!("class1" => false, "class2" => true), "class2");
    }

    #[test]
    fn test_mixed_usage() {
        let dynamic_class = "dynamic";
        assert_eq!(
            clsx!("static", dynamic_class, "conditional" => true),
            "static dynamic conditional"
        );
        assert_eq!(
            clsx!("base", "always", "active" => true, "disabled" => false),
            "base always active"
        );
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(clsx!(), "");
    }

    #[test]
    fn test_complex_usage() {
        let is_active = true;
        let is_disabled = false;
        assert_eq!(
            clsx!("header", "button" => is_active, "hidden" => is_disabled, "footer"),
            "header button footer"
        );
    }

    #[test]
    fn test_trailing_comma() {
        assert_eq!(clsx!("class1", "class2",), "class1 class2");
        assert_eq!(clsx!("class1" => true, "class2" => false,), "class1");
    }
}

//! A set of utility macro for Rust

/// Use `for` loop with C-like syntax in [`cfor!`] block
///
/// # Example
///
/// ```rust
/// # use e_macro::cfor;
/// cfor! {
///     for (let mut i = 0; i < 10; i += 1) {
///         println!("Got: {}", i);
///     }
/// };
/// ```
#[macro_export]
macro_rules! cfor {
    (for (;;) $block: block) => {
        loop $block
    };
    (for (; $condition: expr;) $block: block) => {
        while $condition $block
    };
    (for (;; $increment: expr) $block: block) => {
        let mut _first = true;
        while {
            if _first {
                _first = false;
            } else {
                $increment;
            }
            true
        } $block
    };
    (for ($initializer: stmt;;) $block: block) => {
        $initializer
        loop $block
    };
    (for (; $condition: expr; $increment: expr) $block: block) => {
        let mut _first = true;
        while {
            if _first {
                _first = false;
            } else {
                $increment;
            }
            $condition
        } $block
    };
    (for ($initializer: stmt;; $increment: expr) $block: block) => {
        $initializer
        let mut _first = true;
        while {
            if _first {
                _first = false;
            } else {
                $increment;
            }
            true
        } $block
    };
    (for ($initializer: stmt; $condition: expr;) $block: block) => {
        $initializer
        while $condition $block
    };
    (for ($initializer: stmt; $condition: expr; $increment: expr) $block: block) => {
        $initializer;
        let mut _first = true;
        while {
            if _first {
                _first = false;
            } else {
                $increment;
            }
            $condition
        } $block
    };
}

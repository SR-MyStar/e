//! A set of utility macro for Rust

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Block, Expr, Stmt, Token,
};

struct CStyleForLoop {
    initializer: Option<Stmt>,
    condition: Option<Expr>,
    increment: Option<Expr>,
    loop_block: Block,
}

impl Parse for CStyleForLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let in_parenthesized;
        parenthesized!(in_parenthesized in input);
        let initializer = if let Ok(stmt) = in_parenthesized.parse() {
            Some(stmt)
        } else {
            in_parenthesized.parse::<Token![;]>()?;
            None
        };
        let condition = if let Ok(expr) = in_parenthesized.parse() {
            Some(expr)
        } else {
            None
        };
        in_parenthesized.parse::<Token![;]>()?;
        let increment = if let Ok(expr) = in_parenthesized.parse() {
            Some(expr)
        } else {
            None
        };
        let loop_block = input.parse()?;

        Ok(Self {
            initializer,
            condition,
            increment,
            loop_block,
        })
    }
}

/// Use `for` loop with C-like syntax in [`cfor!`] block
///
/// # Example
///
/// ```rust
/// e_macro::cfor! {
///     for (let mut i = 0; i < 10; i += 1) {
///         println!("Got: {}", i);
///     }
/// };
/// ```
#[proc_macro]
pub fn cfor(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let CStyleForLoop {
        initializer,
        condition,
        increment,
        loop_block,
    } = syn::parse_macro_input!(input_clone as CStyleForLoop);
    TokenStream::from(
        if initializer.is_none() && condition.is_none() && increment.is_none() {
            quote! {
                loop {
                    #loop_block
                }
            }
        } else if initializer.is_none() && condition.is_none() {
            let increment = increment.unwrap();
            quote! {
                let mut _first = true;
                loop {
                    if _first {
                        _first = false;
                    } else {
                        #increment;
                    }
                    #loop_block
                }
            }
        } else if initializer.is_none() && increment.is_none() {
            let condition = condition.unwrap();
            quote! {
                while #condition {
                    #loop_block
                }
            }
        } else if condition.is_none() && increment.is_none() {
            let initializer = initializer.unwrap();
            quote! {
                #initializer
                loop {
                    #loop_block
                }
            }
        } else if initializer.is_none() {
            let (condition, increment) = (condition.unwrap(), increment.unwrap());
            quote! {
                let mut _first = true;
                while {
                    if _first {
                        _first = false;
                    } else {
                        #increment;
                    }
                    #condition
                } {
                    #loop_block
                }
            }
        } else if condition.is_none() {
            let (initializer, increment) = (initializer.unwrap(), increment.unwrap());
            quote! {
                #initializer
                let mut _first = true;
                loop {
                    if _first {
                        _first = false;
                    } else {
                        #increment;
                    }
                    #loop_block
                }
            }
        } else if increment.is_none() {
            let (initializer, condition) = (initializer.unwrap(), condition.unwrap());
            quote! {
                #initializer
                while #condition {
                    #loop_block
                }
            }
        } else {
            let (initializer, condition, increment) =
                (initializer.unwrap(), condition.unwrap(), increment.unwrap());
            quote! {
                #initializer
                let mut _first = true;
                while {
                    if _first {
                        _first = false;
                    } else {
                        #increment;
                    }
                    #condition
                } {
                    #loop_block
                }
            }
        },
    )
}

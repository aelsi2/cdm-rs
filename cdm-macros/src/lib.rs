//! Procedural macros for the cdm-rt crate.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemFn, ReturnType, Type, Visibility, parse, parse_macro_input, spanned::Spanned};

/// Delcares the entry point of the program
///
/// The specified function must have the following signature: `[unsafe] fn() -> !`. 
/// It will be called by the reset handler after initialization. 
///
/// ``` no_run
/// #[cdm_macros::entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let signature_valid = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && f.sig.inputs.len() == 0
        && match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        };

    if !signature_valid {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let name = f.sig.ident;
    let stmts = f.block.stmts;

    quote!(
        #[unsafe(export_name = "main")]
        #(#attrs)*
        pub #unsafety extern "C" fn #name() -> ! {
            #(#stmts)*
        }
    )
    .into()
}

//! Procedural macros for the cdm-rt crate.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Ident, ItemFn, ReturnType, Type,
    parse::{self, Parse},
    parse_macro_input,
    spanned::Spanned,
};

#[derive(Debug, PartialEq)]
enum ExceptionKind {
    Default,
    UnalignedSP,
    UnalignedPC,
    InvalidInst,
    DoubleFault,
}

impl ExceptionKind {
    fn export_name(&self) -> &'static str {
        match self {
            ExceptionKind::Default => "ExceptionHandler",
            ExceptionKind::UnalignedSP => "UnalignedSP",
            ExceptionKind::UnalignedPC => "UnalignedPC",
            ExceptionKind::InvalidInst => "InvalidInst",
            ExceptionKind::DoubleFault => "DoubleFault",
        }
    }
}

impl Default for ExceptionKind {
    fn default() -> Self {
        ExceptionKind::Default
    }
}

impl Parse for ExceptionKind {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "`#[exception(...)]` requires an exception kind",
            ));
        }
        let ident = input.parse::<Ident>()?;
        let kind = match ident.to_string().as_str() {
            "Default" => Self::Default,
            "UnalignedSP" => Self::UnalignedSP,
            "UnalignedPC" => Self::UnalignedPC,
            "InvalidInst" => Self::InvalidInst,
            "DoubleFault" => Self::DoubleFault,
            _ => {
                return Err(syn::Error::new_spanned(
                    ident,
                    "Exception kind must be one of: `Default`, `UnalignedSP`, `UnalignedPC`, `InvalidInst`, `DoubleFault`",
                ));
            }
        };

        Ok(kind)
    }
}

/// Delcares the entry point of the program.
///
/// The function must have the following signature: `[unsafe] fn() -> !`.
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
        return parse::Error::new(Span::call_site(), "`#[entry]` accepts no arguments")
            .to_compile_error()
            .into();
    }

    let attrs = f.attrs;
    let vis = f.vis;
    let unsafety = f.sig.unsafety;
    let name = f.sig.ident;
    let stmts = f.block.stmts;

    quote!(
        #[unsafe(export_name = "main")]
        #(#attrs)*
        #vis #unsafety extern "C" fn #name() -> ! {
            #(#stmts)*
        }
    )
    .into()
}

/// Delcares an exception handler. 
/// 
/// One of the following exception kinds must be specified as a paramter to the attribute:
/// - `Default` - default exception handler used for all exceptions when not overriden by a
/// specific handler
/// - `UnalignedSP` - unaligned stack pointer
/// - `UnalignedPC` - unaligned program counter
/// - `InvalidInst` - invalid instruction
/// - `DoubleFault` - double fault
///
/// The function must have the following signature: `[unsafe] fn() -> !`.
/// It will be called when the specified exception occurs.
///
/// ``` no_run
/// #[cdm_macros::exception(Default)]
/// fn on_exception() -> ! {
///     loop {
///         /* .. */
///     }
/// }
///
/// #[cdm_macros::exception(InvalidInst)]
/// fn on_invalid_inst() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let signature_valid = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
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
            "`#[exception(...)]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    let kind: ExceptionKind = parse_macro_input!(args);

    let export_name = kind.export_name();
    let attrs = f.attrs;
    let vis = f.vis;
    let unsafety = f.sig.unsafety;
    let name = f.sig.ident;
    let stmts = f.block.stmts;

    quote!(
        #[unsafe(export_name = #export_name)]
        #(#attrs)*
        #vis #unsafety extern "C" fn #name() -> ! {
            #(#stmts)*
        }
    )
    .into()
}

/// Delcares an application-specific interrupt handler.
///
/// The function must have the following signature: `[unsafe] fn() -> !`.
///
/// Use `cdm_rt::interrupt_vectors` to register the function in the interrupt vector table.
///
/// ``` no_run
/// cdm_rt::interrupt_vectors![
///     cdm_rt::InterruptVector(on_input, cdm_rt::Psr::None)
/// ];
///
/// #[cdm_macros::interrupt]
/// fn on_input() {
///     /* .. */
/// }
/// ```
#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let signature_valid = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && f.sig.inputs.len() == 0
        && match f.sig.output {
            ReturnType::Default => true,
            _ => false,
        };

    if !signature_valid {
        return parse::Error::new(
            f.span(),
            "`#[interrupt]` function must have signature `[unsafe] fn()`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "`#[interrupt]` attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let vis = f.vis;
    let name = f.sig.ident;
    let stmts = f.block.stmts;

    quote!(
        #(#attrs)*
        #vis #unsafety extern "cdm-isr" fn #name() {
            #(#stmts)*
        }
    )
    .into()
}

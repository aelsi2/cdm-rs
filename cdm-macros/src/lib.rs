//! Procedural macros for the `cdm-rt` crate.

#![doc(html_logo_url = "https://aelsi2.github.io/cdm-rs/logo.png")]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    FnArg, Ident, ItemFn, LitInt, ReturnType, Type, parse::{self, Parse}, parse_macro_input, spanned::Spanned
};

#[derive(Debug, PartialEq)]
enum ExceptionKind {
    Default,
    UnalignedSP,
    UnalignedPC,
    InvalidInst,
    DoubleFault,
    PrivViolation,
    Reserved6,
    SystemCall,
    Reserved8,
    Reserved9,
    ReservedA,
    ReservedB,
    ReservedC,
    ReservedD,
    ReservedE,
    ReservedF,
}

impl ExceptionKind {
    fn export_name(&self) -> &'static str {
        match self {
            ExceptionKind::Default => "_exception_handler",
            ExceptionKind::UnalignedSP => "_ex_unaligned_sp",
            ExceptionKind::UnalignedPC => "_ex_unaligned_pc",
            ExceptionKind::InvalidInst => "_ex_invalid_inst",
            ExceptionKind::DoubleFault => "_ex_double_fault",
            ExceptionKind::PrivViolation => "_ex_priv_violation",
            ExceptionKind::Reserved6 => "_ex_reserved_6",
            ExceptionKind::SystemCall => "_ex_system_call",
            ExceptionKind::Reserved8 => "_ex_reserved_8",
            ExceptionKind::Reserved9 => "_ex_reserved_9",
            ExceptionKind::ReservedA => "_ex_reserved_a",
            ExceptionKind::ReservedB => "_ex_reserved_b",
            ExceptionKind::ReservedC => "_ex_reserved_c",
            ExceptionKind::ReservedD => "_ex_reserved_d",
            ExceptionKind::ReservedE => "_ex_reserved_e",
            ExceptionKind::ReservedF => "_ex_reserved_f",
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
        const MISSING_KIND: &str = "`#[exception(...)]` requires an exception kind";
        const INVALID_KIND: &str = "Exception kind must be a number between 1 and 15 or one of: `Default`, `UnalignedSP`, `UnalignedPC`, `InvalidInst`, `DoubleFault`, `PrivViolation`, `SystemCall`";
        const RESET_FORBIDDEN: &str = "Setting the reset vector is not allowed";

        if input.is_empty() {
            return Err(syn::Error::new(Span::call_site(), MISSING_KIND));
        }

        let kind = if input.lookahead1().peek(LitInt) {
            let literal = input.parse::<LitInt>()?;
            let value: u8 = literal.base10_parse()?;
            match value {
                0 => {
                    return Err(syn::Error::new(Span::call_site(), RESET_FORBIDDEN));
                }
                1 => Self::UnalignedSP,
                2 => Self::UnalignedPC,
                3 => Self::InvalidInst,
                4 => Self::DoubleFault,
                5 => Self::PrivViolation,
                6 => Self::Reserved6,
                7 => Self::SystemCall,
                8 => Self::Reserved8,
                9 => Self::Reserved9,
                10 => Self::ReservedA,
                11 => Self::ReservedB,
                12 => Self::ReservedC,
                13 => Self::ReservedD,
                14 => Self::ReservedE,
                15 => Self::ReservedF,
                _ => {
                    return Err(syn::Error::new_spanned(literal, INVALID_KIND));
                }
            }
        } else {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "Default" => Self::Default,
                "UnalignedSP" => Self::UnalignedSP,
                "UnalignedPC" => Self::UnalignedPC,
                "InvalidInst" => Self::InvalidInst,
                "DoubleFault" => Self::DoubleFault,
                "PrivViolation" => Self::PrivViolation,
                "SystemCall" => Self::SystemCall,
                _ => {
                    return Err(syn::Error::new_spanned(ident, INVALID_KIND));
                }
            }
        };

        Ok(kind)
    }
}

trait ItemFnExt {
    fn validate_isr_sig(&self) -> bool;
    fn validate_entry_sig(&self) -> bool;
}

impl ItemFnExt for ItemFn {
    fn validate_isr_sig(&self) -> bool {
        self.sig.constness.is_none()
            && self.sig.asyncness.is_none()
            && self.sig.abi.is_none()
            && self.sig.generics.params.is_empty()
            && self.sig.generics.where_clause.is_none()
            && self.sig.variadic.is_none()
            && self.sig.inputs.len() <= 1
            && match self.sig.inputs.iter().next() {
                Some(arg) => match arg {
                    FnArg::Receiver(_) => false,
                    FnArg::Typed(pat_type) => match &*pat_type.ty {
                        Type::Infer(_) => true,
                        _ => false,
                    },
                },
                None => true,
            }
            && match self.sig.output {
                ReturnType::Default => true,
                ReturnType::Type(_, ref ty) => match &**ty {
                    Type::Never(_) => true,
                    Type::Tuple(type_tuple) => type_tuple.elems.is_empty(),
                    _ => false,
                }
            }
    }

    fn validate_entry_sig(&self) -> bool {
        self.sig.constness.is_none()
            && self.sig.asyncness.is_none()
            && self.sig.abi.is_none()
            && self.sig.generics.params.is_empty()
            && self.sig.generics.where_clause.is_none()
            && self.sig.variadic.is_none()
            && self.sig.inputs.len() == 0
            && match self.sig.output {
                ReturnType::Default => false,
                ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
            }
    }
}

/// Defines the entry point of the program.
///
/// The function must have the following signature: `[unsafe] fn() -> !`.
/// It will be called by the reset handler after initialization.
///
/// The entry point must be defined once in the entire application, including all crates.
///
/// ``` no_run
/// #[cdm_rt::entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    if !f.validate_entry_sig() {
        return parse::Error::new(
            f.sig.span(),
            "The `#[entry]` function must have the signature `[unsafe] fn() -> !`",
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

/// Defines an exception handler.
///
/// One of the following exception kinds must be specified as a paramter to the attribute:
/// - `Default` - default exception handler used for all exceptions when not overriden by a
/// specific handler
/// - `UnalignedSP` - unaligned stack pointer
/// - `UnalignedPC` - unaligned program counter
/// - `InvalidInst` - invalid instruction
/// - `DoubleFault` - double fault
/// - `PrivViolation` - privilege violation (CdM-16e only)
/// - `SystemCall` - system call (CdM-16e only)
///
/// Alternatively, a integer literal IVT index in the range [1; 15] can be used.
///
/// The function must return `()` or `!` and accept either no parameters or a single parameter of type `_`.
/// The macro replaces the type of the parameter with `&mut ::cdm_rt::InterruptContext`.
/// The parameter can be used to inspect and modify the processor context saved on the stack.
///
/// Each exception handler must be defined at most once in the entire application, including all crates.
///
/// ``` no_run
/// #[cdm_rt::exception(Default)]
/// fn on_exception() -> ! {
///     loop {
///         /* .. */
///     }
/// }
///
/// #[cdm_rt::exception(InvalidInst)]
/// fn on_invalid_inst(ctx: _) -> ! {
///     loop {
///         /* .. */
///     }
/// }
///
/// #[cdm_rt::exception(7)]
/// fn on_system_call(ctx: _) {
///     unsafe { ctx.as_mut().pc += 2 };
///     /* handle system call */
/// }
/// ```
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    if !f.validate_isr_sig() {
        return parse::Error::new(
            f.sig.span(),
            "`#[interrupt]` functions must return either `()` or `!` and accept either no parameters or a single parameter of type `_`",
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
    let ret = f.sig.output;
    let first_arg = f.sig.inputs.iter().next();

    if let Some(arg) = first_arg {
        let FnArg::Typed(pat_type) = arg else { panic!("unreachable") };
        let mut typed_arg = pat_type.clone();
        typed_arg.ty = Box::new(syn::parse_str::<syn::Type>("&mut ::cdm_rt::InterruptContext").unwrap());

        quote!(
            #[unsafe(export_name = #export_name)]
            #(#attrs)*
            #vis #unsafety extern "cdm-isr" fn #name(#typed_arg) #ret {
                #(#stmts)*
            }
        )
    } else {
        quote!(
            #[unsafe(export_name = #export_name)]
            #(#attrs)*
            #vis #unsafety extern "cdm-isr" fn #name(_: &mut ::cdm_rt::InterruptContext) #ret {
                #(#stmts)*
            }
        )
    }.into()
}

/// Defines an application-specific interrupt handler.
///
/// The function must return `()` or `!` and accept either no parameters or a single parameter of type `_`.
/// The macro replaces the type of the parameter with `&mut ::cdm_rt::InterruptContext`.
/// The parameter can be used to inspect and modify the processor context saved on the stack.
///
/// Use the `cdm_rt::interrupt_vectors` macro to register the function in the interrupt vector table.
///
/// ``` no_run
/// cdm_rt::interrupt_vectors![
///     cdm_rt::InterruptVector(on_input, cdm_rt::Psr::NONE),
///     cdm_rt::InterruptVector(on_timer, cdm_rt::Psr::NONE),
/// ];
///
/// #[cdm_rt::interrupt]
/// fn on_input() {
///     /* .. */
/// }
///
/// #[cdm_rt::interrupt]
/// fn on_timer(ctx: _) {
///     /* .. */
/// }
/// ```
#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    if !args.is_empty() {
        return parse::Error::new(
           Span::call_site(),
           "`#[interrupt]` attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    if !f.validate_isr_sig() {
        return parse::Error::new(
            f.sig.span(),
            "`#[interrupt]` functions must return either `()` or `!` and accept either no parameters or a single parameter of type `_`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let vis = f.vis;
    let unsafety = f.sig.unsafety;
    let name = f.sig.ident;
    let stmts = f.block.stmts;
    let ret = f.sig.output;
    let first_arg = f.sig.inputs.iter().next();

    if let Some(arg) = first_arg {
        let FnArg::Typed(pat_type) = arg else { panic!("receiver argument should have been rejected by validate_isr_sig") };
        let mut pat_type = pat_type.clone();
        pat_type.ty = Box::new(syn::parse_str::<syn::Type>("&mut ::cdm_rt::InterruptContext").unwrap());

        quote!(
            #(#attrs)*
            #vis #unsafety extern "cdm-isr" fn #name(#pat_type) #ret {
                #(#stmts)*
            }
        )
    } else {
        quote!(
            #(#attrs)*
            #vis #unsafety extern "cdm-isr" fn #name(_: &mut ::cdm_rt::InterruptContext) #ret {
                #(#stmts)*
            }
        )
    }
    .into()
}

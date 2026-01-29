//! Internal implementation details of `cortex-m-rt` (ARM9 version).

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashSet;
use syn::{
    parse::{self},
    parse_macro_input,
    spanned::Spanned,
    AttrStyle, Attribute, FnArg, Ident, Item, ItemFn, ItemStatic, ReturnType, Stmt, Type,
    Visibility,
};

fn is_inherited(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Inherited)
}

fn is_outer_attr(style: &AttrStyle) -> bool {
    matches!(style, AttrStyle::Outer)
}

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    let valid_signature = f.sig.constness.is_none()
        && is_inherited(&f.vis)
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        };

    if !valid_signature {
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

    let (statics, stmts) = match extract_static_muts(f.block.stmts) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    f.sig.ident = Ident::new(&format!("__cortex_m_rt_{}", f.sig.ident), Span::call_site());
    f.sig.inputs.extend(statics.iter().map(|statik| {
        let ident = &statik.ident;
        let ty = &statik.ty;
        let attrs = &statik.attrs;
        syn::parse::<FnArg>(
            quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &'static mut #ty).into(),
        )
        .unwrap()
    }));
    f.block.stmts = stmts;

    let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());
    let ident = &f.sig.ident;

    let resource_args = statics
        .iter()
        .map(|statik| {
            let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
            let ident = &statik.ident;
            let ty = &statik.ty;
            let expr = &statik.expr;
            quote! {
                #(#cfgs)*
                {
                    #(#attrs)*
                    static mut #ident: #ty = #expr;
                    unsafe { &mut #ident }
                }
            }
        })
        .collect::<Vec<_>>();

    if let Err(error) = check_attr_whitelist(&f.attrs, WhiteListCaller::Entry) {
        return error;
    }

    let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

    quote!(
        #(#cfgs)*
        #(#attrs)*
        #[doc(hidden)]
        #[export_name = "main"]
        pub unsafe extern "C" fn #tramp_ident() {
            #[allow(static_mut_refs)]
            #ident(
                #(#resource_args),*
            )
        }

        #f
    )
    .into()
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Exception {
    Undefined,
    SWI,
    PrefetchAbort,
    DataAbort,
    IRQ,
    FIQ,
}

/// ARM9 exception handler attribute
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    if let Err(error) = check_attr_whitelist(&f.attrs, WhiteListCaller::Exception) {
        return error;
    }

    let fspan = f.span();
    let ident = f.sig.ident.clone();
    let ident_s = ident.to_string();

    let _exn = match ident_s.as_str() {
        "Undefined" => Exception::Undefined,
        "SWI" => Exception::SWI,
        "PrefetchAbort" => Exception::PrefetchAbort,
        "DataAbort" => Exception::DataAbort,
        "IRQ" => Exception::IRQ,
        "FIQ" => Exception::FIQ,
        _ => {
            return parse::Error::new(
                ident.span(),
                "Invalid ARM9 exception. Valid: Undefined, SWI, PrefetchAbort, DataAbort, IRQ, FIQ",
            )
            .to_compile_error()
            .into();
        }
    };

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && is_inherited(&f.vis)
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                Type::Never(..) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            fspan,
            "`#[exception]` handlers must have signature `[unsafe] fn() [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let (statics, stmts) = match extract_static_muts(f.block.stmts) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    let export_ident = f.sig.ident.clone();
    let internal_ident = Ident::new(&format!("__cortex_m_rt_{}", f.sig.ident), Span::call_site());
    f.sig.ident = internal_ident.clone();

    f.sig.inputs.extend(statics.iter().map(|statik| {
        let ident = &statik.ident;
        let ty = &statik.ty;
        let attrs = &statik.attrs;
        syn::parse::<FnArg>(quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &mut #ty).into())
            .unwrap()
    }));
    f.block.stmts = stmts;

    let resource_args = statics
        .iter()
        .map(|statik| {
            let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
            let ident = &statik.ident;
            let ty = &statik.ty;
            let expr = &statik.expr;
            quote! {
                #(#cfgs)*
                {
                    #(#attrs)*
                    static mut #ident: #ty = #expr;
                    unsafe { &mut #ident }
                }
            }
        })
        .collect::<Vec<_>>();

    let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

    quote!(
        #(#cfgs)*
        #(#attrs)*
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn #export_ident() {
            #[allow(static_mut_refs)]
            #internal_ident(
                #(#resource_args),*
            )
        }

        #f
    )
    .into()
}

fn extract_static_muts(
    stmts: impl IntoIterator<Item = Stmt>,
) -> Result<(Vec<ItemStatic>, Vec<Stmt>), parse::Error> {
    let mut istmts = stmts.into_iter();
    let mut seen = HashSet::new();
    let mut statics = vec![];
    let mut stmts = vec![];

    for stmt in istmts.by_ref() {
        match stmt {
            Stmt::Item(Item::Static(var)) => match var.mutability {
                syn::StaticMutability::Mut(_) => {
                    if seen.contains(&var.ident) {
                        return Err(parse::Error::new(
                            var.ident.span(),
                            format!("the name `{}` is defined multiple times", var.ident),
                        ));
                    }
                    seen.insert(var.ident.clone());
                    statics.push(var);
                }
                _ => stmts.push(Stmt::Item(Item::Static(var))),
            },
            _ => {
                stmts.push(stmt);
                break;
            }
        }
    }

    stmts.extend(istmts);
    Ok((statics, stmts))
}

fn extract_cfgs(attrs: Vec<Attribute>) -> (Vec<Attribute>, Vec<Attribute>) {
    let mut cfgs = vec![];
    let mut not_cfgs = vec![];

    for attr in attrs {
        if eq(&attr, "cfg") {
            cfgs.push(attr);
        } else {
            not_cfgs.push(attr);
        }
    }

    (cfgs, not_cfgs)
}

enum WhiteListCaller {
    Entry,
    Exception,
}

fn check_attr_whitelist(attrs: &[Attribute], caller: WhiteListCaller) -> Result<(), TokenStream> {
    let whitelist = &[
        "doc", "link_section", "cfg", "allow", "warn", "deny", "forbid", "cold", "naked", "expect",
    ];

    'o: for attr in attrs {
        if let Some(attr_name) = get_attr_name(attr) {
            if whitelist.contains(&attr_name.as_str()) {
                continue 'o;
            }
        }

        let err_str = match caller {
            WhiteListCaller::Entry => "this attribute is not allowed on entry point",
            WhiteListCaller::Exception => "this attribute is not allowed on exception handler",
        };

        return Err(parse::Error::new(attr.span(), err_str)
            .to_compile_error()
            .into());
    }

    Ok(())
}

fn eq(attr: &Attribute, name: &str) -> bool {
    is_outer_attr(&attr.style) && attr.path().is_ident(name)
}

fn get_attr_name(attr: &Attribute) -> Option<String> {
    if !is_outer_attr(&attr.style) {
        return None;
    }

    let name = attr.path().get_ident().map(|x| x.to_string());

    match &name {
        Some(name) if name == "unsafe" => {
            if let Ok(inner_meta) = attr.parse_args::<syn::Meta>() {
                inner_meta.path().get_ident().map(|x| x.to_string())
            } else {
                None
            }
        }
        _ => name,
    }
}

#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, AttrStyle, Expr, Ident, ItemMod, ItemUse, Meta, Signature};

#[proc_macro_attribute]
/// Annotate `use` declarations with this to replace them in syncified output.
pub fn syncify_replace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
/// Produce a sync version of the anotated async function by stripping the `async` keyword and all `.await`s.
pub fn syncify(attr: TokenStream, item: TokenStream) -> TokenStream {
    let sync_mod_name = parse_macro_input!(attr as Ident);

    let input_mod = parse_macro_input!(item as ItemMod);

    let mut sync_copy = input_mod.clone();
    sync_copy.ident = sync_mod_name;
    SyncifyVisitor.visit_item_mod_mut(&mut sync_copy);

    let mut out = proc_macro2::TokenStream::new();
    input_mod.to_tokens(&mut out);
    sync_copy.to_tokens(&mut out);
    out.into()
}

struct SyncifyVisitor;

impl VisitMut for SyncifyVisitor {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i {
            Expr::Await(exp) => {
                *i = *exp.base.clone();
            }
            _ => visit_mut::visit_expr_mut(self, i),
        }
    }

    fn visit_signature_mut(&mut self, i: &mut Signature) {
        i.asyncness = None;
    }

    fn visit_item_use_mut(&mut self, i: &mut syn::ItemUse) {
        // Look for `syncify_replace` attributes on `use` items, and honour them.
        for attr in i.attrs.clone().iter() {
            // Ignore inner attributes
            if matches!(attr.style, AttrStyle::Outer) {
                match &attr.meta {
                    Meta::List(meta_list) => {
                        if let Some(ident) = meta_list.path.get_ident() {
                            if ident == "syncify_replace" {
                                let new_use: ItemUse = syn::parse2(meta_list.tokens.clone()).expect("Content of the syncify_replace macro must be a valid `use` item, for example `use std::collections::HashSet`.");
                                *i = new_use;
                            }
                        }
                    }
                    _ => { /* no-op */ }
                }
            }
        }
    }
}

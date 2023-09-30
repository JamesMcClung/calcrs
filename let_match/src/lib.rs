use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{self, Expr, Pat, Token};

struct LetMatchInput {
    pat: Pat,
    _comma: Token![,],
    expr: Expr,
}

impl Parse for LetMatchInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { pat: Pat::parse_multi_with_leading_vert(input)?, _comma: input.parse()?, expr: input.parse()? })
    }
}

#[proc_macro]
pub fn let_match(input: TokenStream) -> TokenStream {
    let LetMatchInput { pat, expr, .. } = syn::parse::<LetMatchInput>(input).expect("let_match error: invalid inputs");
    let_match_impl(pat, expr)
}

fn let_match_impl(mut pat: Pat, expr: Expr) -> TokenStream {
    let idents = find_identifiers(&mut pat);
    let ident_assign_nones = idents
        .iter()
        .map(|ident| {
            quote! {
                let mut #ident = None;
            }
        })
        .reduce(extend_reducer)
        .expect("let_match error: ident required");
    let ident_assign_somes = idents
        .iter()
        .map(|ident| {
            let ident_underscore = Ident::new(&format!("{}_", ident.to_string()), Span::call_site());
            quote! {
                #ident = Some(#ident_underscore);
            }
        })
        .reduce(extend_reducer)
        .expect("let_match error: ident required");
    let ident_unwraps = idents
        .iter()
        .map(|ident| {
            quote! {
                let #ident = #ident.expect("let_match error: pattern failed to match");
            }
        })
        .reduce(extend_reducer)
        .expect("let_match error: ident required");
    quote! {
        #ident_assign_nones
        if let #pat = (#expr) {
            #ident_assign_somes
        }
        #ident_unwraps
    }
    .into()
}

fn extend_reducer<S, T: Extend<S> + IntoIterator<Item = S>>(mut accumulator: T, next: T) -> T {
    accumulator.extend(next);
    accumulator
}

fn find_identifiers(pat: &mut Pat) -> Vec<Ident> {
    let mut ids = Vec::new();
    find_identifiers_impl(pat, &mut ids);
    ids
}

fn find_identifiers_impl(pat: &mut Pat, ids: &mut Vec<Ident>) {
    match pat {
        Pat::Ident(pat) => {
            let ident_underscore = format!("{}_", pat.ident.to_string());
            ids.push(std::mem::replace(&mut pat.ident, Ident::new(&ident_underscore, Span::call_site())));
        },
        Pat::Tuple(pat) => pat.elems.pairs_mut().for_each(|mut pair| find_identifiers_impl(pair.value_mut(), ids)),
        Pat::TupleStruct(pat) => pat.elems.pairs_mut().for_each(|mut pair| find_identifiers_impl(pair.value_mut(), ids)),
        Pat::Struct(pat) => pat.fields.pairs_mut().for_each(|mut pair| {
            pair.value_mut().colon_token = Some(syn::token::Colon([Span::call_site()]));
            find_identifiers_impl(&mut pair.value_mut().pat, ids);
        }),
        Pat::Slice(pat) => pat.elems.pairs_mut().for_each(|mut pair| find_identifiers_impl(pair.value_mut(), ids)),
        _ => (),
    }
}

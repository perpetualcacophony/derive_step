#![feature(step_trait)]
#![feature(iter_map_windows)]

use manyhow::{bail, ensure, manyhow, Result};
use syn::spanned::Spanned;

#[manyhow(proc_macro_derive(Step))]
pub fn derive_step(input: syn::DeriveInput) -> Result<syn::ItemImpl> {
    if let syn::Data::Enum(ref data) = input.data {
        ensure!(
            !data.variants.is_empty(),
            "Using this derive for an empty enum is unsupported."
        );

        ensure!(
            data.variants
                .iter()
                .all(|variant| variant.fields.is_empty()),
            "Using this derive for an enum with variant fields is unsupported."
        );

        let successors: Vec<syn::Arm> = data
            .variants
            .iter()
            .map_windows(|variants: &[&syn::Variant; 2]| {
                let start = &variants[0].ident;
                let successor = &variants[1].ident;

                syn::parse_quote_spanned! {variants[0].span()=>
                    Self::#start => Some(Self::#successor)
                }
            })
            .collect();

        let predecessors: Vec<syn::Arm> = data
            .variants
            .iter()
            .skip(1)
            .map_windows(|variants: &[&syn::Variant; 2]| {
                let start = &variants[1].ident;
                let predecessor = &variants[0].ident;

                syn::parse_quote_spanned! {variants[1].span()=>
                    Self::#start => Some(Self::#predecessor)
                }
            })
            .collect();

        let name = &input.ident;

        let expanded = syn::parse_quote_spanned!(input.span()=>
            impl std::iter::Step for #name {
                fn forward_checked(start: Self, count: usize) -> Option<Self> {
                    let next = match start {
                        #(#successors),*,
                        _ => None
                    };

                    if let Some(next) = next {
                        if count == 1 {
                            Some(next)
                        } else {
                            Self::forward_checked(next, count - 1)
                        }
                    } else {
                        None
                    }
                }

                fn backward_checked(start: Self, count: usize) -> Option<Self> {
                    let next = match start {
                        #(#predecessors),*,
                        _ => None
                    };

                    if let Some(next) = next {
                        if count == 1 {
                            Some(next)
                        } else {
                            Self::backward_checked(next, count - 1)
                        }
                    } else {
                        None
                    }
                }

                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if start > end { return None; }
                    if start == end { return Some(0); }

                    let mut counter = 1;
                    let mut current = Self::forward_checked(start.clone(), 1);

                    while current.as_ref() != Some(end) && current.is_some() {
                        counter += 1;
                        current = Self::forward_checked(current.unwrap(), 1);
                    }

                    if current.as_ref() == Some(end) {
                        return Some(counter);
                    } else {
                        return None;
                    }
                }
            }
        );

        Ok(expanded)
    } else {
        bail!("Using this derive for anything other than an enum is currently unsupported.")
    }
}

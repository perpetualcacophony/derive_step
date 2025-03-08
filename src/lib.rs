#![no_std]
#![feature(step_trait)]
#![feature(iter_map_windows)]

use manyhow::{bail, ensure, manyhow};
use syn::spanned::Spanned;

#[manyhow(proc_macro_derive(Step))]
pub fn derive_step(input: syn::DeriveInput) -> manyhow::Result<syn::ItemImpl> {
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

        ensure!(
            data.variants.len() != 1,
            "Using this derive for an enum with a single variant is unsupported."
        );

        ensure!(data.variants.len() != 2, "Using this derive for an enum with only two variants is currently unsupported. If you happen to need this behavior, please open an issue! I don't currently see a use, and I don't feel like hunting down the nasty bug it's causing right now.");

        let successors =
            data.variants
                .iter()
                .map_windows(|variants: &[&syn::Variant; 2]| -> syn::Arm {
                    let start = &variants[0].ident;
                    let successor = &variants[1].ident;

                    syn::parse_quote_spanned! {variants[0].span()=>
                        Self::#start => Some(Self::#successor)
                    }
                });

        let predecessors =
            data.variants
                .iter()
                .rev()
                .map_windows(|variants: &[&syn::Variant; 2]| -> syn::Arm {
                    let start = &variants[0].ident;
                    let predecessor = &variants[1].ident;

                    syn::parse_quote_spanned! {variants[0].span()=>
                        Self::#start => Some(Self::#predecessor)
                    }
                });

        let name = &input.ident;

        let expanded = syn::parse_quote_spanned!(input.span()=>
            impl std::iter::Step for #name {
                fn forward_checked(start: Self, count: usize) -> Option<Self> {
                    if count == 0 {
                        return Some(start)
                    }

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
                    if count == 0 {
                        return Some(start)
                    }

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

                fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                    if start > end { return (0, None); }
                    if start == end { return (0, Some(0)); }

                    let mut counter = 1;
                    let mut current = Self::forward_checked(start.clone(), 1);

                    while current.as_ref() != Some(end) && current.is_some() {
                        counter += 1;
                        current = Self::forward_checked(current.unwrap(), 1);
                    }

                    if current.as_ref() == Some(end) {
                        return (counter, Some(counter));
                    } else {
                        return (0, None);
                    }
                }
            }
        );

        Ok(expanded)
    } else {
        bail!("Using this derive for anything other than an enum is currently unsupported.")
    }
}

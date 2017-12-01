#![feature(proc_macro)]

             extern crate proc_macro;
#[macro_use] extern crate quote;
             extern crate simulacrum;
             extern crate syn;

use proc_macro::TokenStream;
use simulacrum::*;

use std::str::FromStr;

fn print_funcs(funcs: Vec<syn::TraitItem>) {
    for func in funcs {
        println!("{:?}", func.ident);
    }
}

fn get_trait_items(item: &syn::Item) -> Vec<syn::TraitItem> {
    match item.node {
        syn::ItemKind::Trait(unsafety, ref generics, ref ty_param_bound, ref items) => {
            items.clone()
        },
        _ => vec![].clone()
    }
}

fn simulacrum_internal(input: &str) -> quote::Tokens {
    // Generate the AST from the token stream we were given
    let item = syn::parse_item(&input.to_string()).unwrap();

    // Generate struct name
    let ident = &item.ident;
    let name = quote! { #ident };
    let name = syn::Ident::new(format!("{}Mock", name.as_str()));

    // Print out function information
    let items = get_trait_items(&item);
    print_funcs(items);

    let output = quote! {
        #item

        pub struct #name {
            e: Expectations
        }

        impl #name {
            pub fn new() -> Self {
                Self {
                    e: Expectations::new()
                }
            }
        }
    };

    output
}

#[proc_macro_attribute]
pub fn simulacrum(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Generate the Rust code string to use as the output
    let output = simulacrum_internal(&input.to_string());

    // Turn that Rust back into a token stream
    TokenStream::from_str(output.as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = quote! {
            pub trait CoolTrait {
                // Shared self
                fn foo(&self);

                // Mutable self
                fn bar(&mut self);

                // One parameter and returning a value
                fn goop(&mut self, flag: bool) -> u32;

                // Multiple parameters
                fn zing(&self, first: i32, second: bool);

                // Note: It doesn't work with references yet!
                // fn boop(&self, name: &'static str)
            }
        };

        let expected = quote! {
            pub trait CoolTrait {
                fn foo(&self);
                fn bar(&mut self);
                fn goop(&mut self, flag: bool) -> u32;
                fn zing(&self, first: i32, second: bool);
            }

            pub struct CoolTraitMock {
                e: Expectations
            }

            impl CoolTraitMock {
                pub fn new() -> Self {
                    Self {
                        e: Expectations::new()
                    }
                }
            }
        };

        let result = simulacrum_internal(input.as_str());

        assert_eq!(result, expected);
    }
}

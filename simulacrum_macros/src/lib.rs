#![feature(proc_macro)]
#![recursion_limit="128"]

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

fn generate_expects(trait_items: &Vec<syn::TraitItem>) -> quote::Tokens {
    let mut result = quote::Tokens::new();
    for item in trait_items {
        match item.node {
            syn::TraitItemKind::Method(ref sig, _) => {
                let ident = &item.ident;
                let name = expectify_method_name(&ident);
                let otype = generate_output_type(&sig.decl.output);
                let itypes = &sig.decl.inputs;
                let expect_method = quote! {
                    pub fn #name(&mut self) -> Method<(), #otype> {
                        self.e.expect::<(), #otype>("#ident")
                    }
                };
                result.append(expect_method)
            },
            _ => { }
        }
    }
    result
}

fn expectify_method_name(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(format!("expect_{}", ident))
}

fn generate_output_type(ret_type: &syn::FunctionRetTy) -> quote::Tokens {
    match ret_type {
        &syn::FunctionRetTy::Default => quote! { () },
        &syn::FunctionRetTy::Ty(ref ty) => quote! { #ty }
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
    let trait_item = get_trait_items(&item);
    let expects = generate_expects(&trait_item);

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

            pub fn then(&mut self) -> &mut Self {
                self.e.then();
                self
            }

            #expects
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
    #[ignore]
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

                pub fn then(&mut self) -> &mut Self {
                    self.e.then();
                    self
                }

                pub fn expect_foo(&mut self) -> Method<(), ()> {
                    self.e.expect::<(), ()>("foo")
                }

                pub fn expect_bar(&mut self) -> Method<(), ()> {
                    self.e.expect::<(), ()>("bar")
                }

                pub fn expect_goop(&mut self) -> Method<bool, u32> {
                    self.e.expect::<bool, u32>("goop")
                }

                pub fn expect_zing(&mut self) -> Method<(i32, bool), ()> {
                    self.e.expect::<(i32, bool), ()>("zing")
                }
            }
        };

        let result = simulacrum_internal(input.as_str());

        assert_eq!(result, expected);
    }
}

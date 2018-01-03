#![feature(proc_macro)]
#![recursion_limit="128"]

             extern crate proc_macro;
#[macro_use] extern crate quote;
             extern crate simulacrum_macros;
             extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;

use std::str::FromStr;

struct Method {
    ident: syn::Ident,
    original_item: syn::TraitItem,
    sig: syn::MethodSig
}

#[proc_macro_attribute]
pub fn simulacrum(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Generate the Rust code string to use as the output
    let output = simulacrum_internal(&input.to_string());

    // Turn that Rust back into a token stream
    TokenStream::from_str(output.as_str()).unwrap()
}

fn simulacrum_internal(input: &str) -> quote::Tokens {
    // Generate the AST from the token stream we were given
    let item = syn::parse_item(&input.to_string()).unwrap();

    // Generate Mock struct name
    let ident = &item.ident;
    let name = quote! { #ident };
    let name = syn::Ident::new(format!("{}Mock", name.as_str()));

    // Get method information
    let trait_items = get_trait_items(&item);
    let methods = gather_trait_methods(&trait_items);

    // Generate what is required for the create_mock! macro
    let annotations = generate_annotations(&methods);
    let original_methods = gather_original_methods(&methods);

    // Generate fn expect_blah() -> Method methods
    // let expects = generate_expects(&methods);

    // Generate blah() stub methods
    // let stubs = generate_stubs(&methods);

    let output = quote! {
        #item

        create_mock! {
            impl #ident for #name (self) {
                #(
                    #annotations
                    #original_methods
                )*
            }
        }
    };

    output
}

fn get_trait_items(item: &syn::Item) -> Vec<syn::TraitItem> {
    match item.node {
        syn::ItemKind::Trait(_unsafety, ref _generics, ref _ty_param_bound, ref items) => {
            items.clone()
        },
        _ => vec![].clone()
    }
}

fn gather_trait_methods(trait_items: &Vec<syn::TraitItem>) -> Vec<Method> {
    let mut result = Vec::new();
    for item in trait_items {
        match item.node {
            syn::TraitItemKind::Method(ref sig, _) => {
                let m = Method {
                    ident: item.ident.clone(),
                    original_item: item.clone(),
                    sig: sig.clone()
                };
                result.push(m);
            },
            _ => { }
        }
    }
    result
}

fn generate_annotations(methods: &Vec<Method>) -> Vec<quote::Tokens> {
    let mut result = Vec::new();
    for method in methods {
        let ident = &method.ident;
        let ident_tokens = quote!{ #ident };
        let ident_str = ident_tokens.as_str();
        let name = expectify_method_name(ident);
        // let otype = generate_output_type(&method.sig.decl.output);
        // let ituple = generate_input_tuple(&method.sig.decl.inputs);
        let annotation = quote! {
            #name(#ident_str):
        };
        result.push(annotation);
    }
    result
}

// fn generate_expects(methods: &Vec<Method>) -> quote::Tokens {
//     let mut result = quote::Tokens::new();
//     for method in methods {
//         let ident = &method.ident;
//         let ident_tokens = quote!{ #ident };
//         let ident_str = ident_tokens.as_str();
//         let name = expectify_method_name(ident);
//         let otype = generate_output_type(&method.sig.decl.output);
//         let ituple = generate_input_tuple(&method.sig.decl.inputs);
//         let expect_method = quote! {
//             pub fn #name(&mut self) -> Method<#ituple, #otype> {
//                 self.e.expect::<#ituple, #otype>(#ident_str)
//             }
//         };
//         result.append(expect_method);
//     }
//     result
// }

fn expectify_method_name(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(format!("expect_{}", ident))
}

// fn generate_output_type(output: &syn::FunctionRetTy) -> quote::Tokens {
//     match output {
//         &syn::FunctionRetTy::Default => quote! { () },
//         &syn::FunctionRetTy::Ty(ref ty) => quote! { #ty }
//     }
// }

// fn generate_input_tuple(input: &Vec<syn::FnArg>) -> quote::Tokens {
//     let types = gather_captured_arg_types(input);

//     let mut result = quote::Tokens::new();
//     let num_types = types.len();
//     match num_types {
//         1 => {
//             let first = types.first().unwrap();
//             first.to_tokens(&mut result);
//         },
//         _ => {
//             result.append("(");
//             let mut num_added = 0;
//             for ty in types {
//                 ty.to_tokens(&mut result);
//                 num_added += 1;

//                 if num_added < num_types {
//                     result.append(",");
//                 }
//             }
//             result.append(")");
//         }
//     }
//     result
// }

// fn gather_captured_arg_types(input: &Vec<syn::FnArg>) -> Vec<syn::Ty> {
//     let mut result = Vec::new();
//     for arg in input {
//         match arg {
//             &syn::FnArg::Captured(ref _pattern, ref ty) => {
//                 result.push(ty.clone());
//             },
//             _ => { }
//         }
//     }
//     result
// }

fn gather_original_methods(methods: &Vec<Method>) -> Vec<quote::Tokens> {
    let mut result = Vec::new();
    for method in methods {
        let to_output = &method.original_item;

        if method_needs_side_effect_added(&method.sig) {
            unimplemented!()
        }

        // Push the tokens onto our result Vec
        let tokens = quote! {
            #to_output
        };
        result.push(tokens);
    }
    result
}

fn method_needs_side_effect_added(sig: &syn::MethodSig) -> bool {
    // If any of the parameters are &mut (excluding &mut self), then we need to
    // have a side-effect added to this method in order to modify those params.
    let args = &sig.decl.inputs;
    for arg in args {
        match arg {
            &syn::FnArg::Captured(_, ref ty) => {
                match ty {
                    &syn::Ty::Ptr(ref mut_ty) => {
                        if mut_ty.mutability == syn::Mutability::Mutable {
                            return true;
                        }
                    },
                    &syn::Ty::Rptr(_, ref mut_ty) => {
                        if mut_ty.mutability == syn::Mutability::Mutable {
                            return true;
                        }
                    },
                    otherwise @ _ => { }
                }
            },
            _ => { }
        }
    }

    // Otherwise, no side-effect needs to be added.
    false
}

// fn generate_stubs(methods: &Vec<Method>) -> quote::Tokens {
//     let mut result = quote::Tokens::new();
//     for method in methods {
//         let ident = &method.ident;
//         let ident_tokens = quote!{ #ident };
//         let ident_str = ident_tokens.as_str();
//         let otype = generate_output_type(&method.sig.decl.output);
//         let ituple = generate_input_tuple(&method.sig.decl.inputs);
//         let itypes = generate_input_types(&method.sig.decl.inputs);
//         // TODO: generate method sig tokens 
//         let mut method_sig_tokens = quote::Tokens::new();
//         // TODO: generate method sig tokens 
//         let mut body_tokens = quote::Tokens::new();
//         let stub_method = quote! {
//             #method_sig_tokens {
//                 #body_tokens
//             }
//         };
//         result.append(stub_method);
//     }
//     result
// }

// fn generate_input_types(input: &Vec<syn::FnArg>) -> quote::Tokens {
//     let mut result = quote::Tokens::new();
//     // for arg in input {
//     //     match arg {
//     //         &syn::FnArg::Captured(ref _pattern, ref ty) => {
//     //             result.push(ty.clone());
//     //         },
//     //         _ => { }
//     //     }
//     // }
//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // // Test for fn blah()
    // fn test_generate_input_tuple_none() {
    //     let input = Vec::new();

    //     let expected = quote! { () };

    //     let result = generate_input_tuple(&input);

    //     assert_eq!(expected, result);
    // }

    // #[test]
    // // Test for fn blah(arg: i32)
    // fn test_generate_input_tuple_one_captured() {
    //     let mut input = Vec::new();
    //     // arg: i32
    //     let binding_mode = syn::BindingMode::ByValue(syn::Mutability::Immutable);
    //     let ident = syn::parse_ident("arg").unwrap();
    //     let pattern = syn::Pat::Ident(binding_mode, ident, None);
    //     let ty = syn::parse_type("i32").unwrap();
    //     let arg = syn::FnArg::Captured(pattern, ty);
    //     input.push(arg);

    //     let expected = quote! { i32 };

    //     let result = generate_input_tuple(&input);

    //     assert_eq!(expected, result);
    // }

    // #[test]
    // // Test for fn blah(&self)
    // fn test_generate_input_tuple_self_ref() {
    //     let mut input = Vec::new();
    //     // &self
    //     let arg = syn::FnArg::SelfRef(None, syn::Mutability::Immutable);
    //     input.push(arg);

    //     let expected = quote! { () };

    //     let result = generate_input_tuple(&input);

    //     assert_eq!(expected, result);
    // }

    // #[test]
    // // Test for fn blah(&self, arg: i32)
    // fn test_generate_input_tuple_self_ref_one_captured() {
    //     let mut input = Vec::new();
    //     // &self
    //     let arg = syn::FnArg::SelfRef(None, syn::Mutability::Immutable);
    //     input.push(arg);
    //     // arg: i32
    //     let binding_mode = syn::BindingMode::ByValue(syn::Mutability::Immutable);
    //     let ident = syn::parse_ident("arg").unwrap();
    //     let pattern = syn::Pat::Ident(binding_mode, ident, None);
    //     let ty = syn::parse_type("i32").unwrap();
    //     let arg = syn::FnArg::Captured(pattern, ty);
    //     input.push(arg);

    //     let expected = quote! { i32 };

    //     let result = generate_input_tuple(&input);

    //     assert_eq!(expected, result);
    // }

    // #[test]
    // // Test for fn blah(&self, arg1: i32, arg2: bool)
    // fn test_generate_input_tuple_self_ref_two_captured() {
    //     let mut input = Vec::new();
    //     // &self
    //     let arg = syn::FnArg::SelfRef(None, syn::Mutability::Immutable);
    //     input.push(arg);
    //     // arg1: i32
    //     let binding_mode = syn::BindingMode::ByValue(syn::Mutability::Immutable);
    //     let ident = syn::parse_ident("arg1").unwrap();
    //     let pattern = syn::Pat::Ident(binding_mode, ident, None);
    //     let ty = syn::parse_type("i32").unwrap();
    //     let arg = syn::FnArg::Captured(pattern, ty);
    //     input.push(arg);
    //     // arg2: bool
    //     let binding_mode = syn::BindingMode::ByValue(syn::Mutability::Immutable);
    //     let ident = syn::parse_ident("arg2").unwrap();
    //     let pattern = syn::Pat::Ident(binding_mode, ident, None);
    //     let ty = syn::parse_type("bool").unwrap();
    //     let arg = syn::FnArg::Captured(pattern, ty);
    //     input.push(arg);

    //     let expected = quote! { ( i32, bool ) };

    //     let result = generate_input_tuple(&input);

    //     assert_eq!(expected, result);
    // }

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
                fn zing(&self, first: i32, second: &mut bool);

                // Note: It doesn't work with references yet!
                // fn boop(&self, name: &'static str)
            }
        };

        let expected = quote! {
            pub trait CoolTrait {
                fn foo(&self);
                fn bar(&mut self);
                fn goop(&mut self, flag: bool) -> u32;
                fn zing(&self, first: i32, second: &mut bool);
            }

            create_mock! {
                impl CoolTrait for CoolTraitMock (self) {
                    expect_foo("foo"):
                    fn foo(&self);

                    expect_bar("bar"):
                    fn bar(&mut self);

                    expect_goop("goop"):
                    fn goop(&mut self, flag: bool) -> u32;

                    expect_zing("zing"):
                    fn zing(&self, first: i32, second: &mut bool) -> ();
                }
            }
        };

        let result = simulacrum_internal(input.as_str());

        assert_eq!(expected, result);
    }

    // #[test]
    // #[ignore]
    // fn it_works() {
    //     let input = quote! {
    //         pub trait CoolTrait {
    //             // Shared self
    //             fn foo(&self);

    //             // Mutable self
    //             fn bar(&mut self);

    //             // One parameter and returning a value
    //             fn goop(&mut self, flag: bool) -> u32;

    //             // Multiple parameters
    //             fn zing(&self, first: i32, second: bool);

    //             // Note: It doesn't work with references yet!
    //             // fn boop(&self, name: &'static str)
    //         }
    //     };

    //     let expected = quote! {
    //         pub trait CoolTrait {
    //             fn foo(&self);
    //             fn bar(&mut self);
    //             fn goop(&mut self, flag: bool) -> u32;
    //             fn zing(&self, first: i32, second: bool);
    //         }

    //         pub struct CoolTraitMock {
    //             e: Expectations
    //         }

    //         impl CoolTraitMock {
    //             pub fn new() -> Self {
    //                 Self {
    //                     e: Expectations::new()
    //                 }
    //             }

    //             pub fn then(&mut self) -> &mut Self {
    //                 self.e.then();
    //                 self
    //             }

    //             pub fn expect_foo(&mut self) -> Method<(), ()> {
    //                 self.e.expect::<(), ()>("foo")
    //             }

    //             pub fn expect_bar(&mut self) -> Method<(), ()> {
    //                 self.e.expect::<(), ()>("bar")
    //             }

    //             pub fn expect_goop(&mut self) -> Method<bool, u32> {
    //                 self.e.expect::<bool, u32>("goop")
    //             }

    //             pub fn expect_zing(&mut self) -> Method<(i32, bool), ()> {
    //                 self.e.expect::<(i32, bool), ()>("zing")
    //             }
    //         }

    //         impl CoolTrait for CoolTraitMock {
    //             fn foo(&self) {
    //                 self.e.was_called::<(), ()>("foo", ())
    //             }

    //             fn bar(&mut self) {
    //                 self.e.was_called::<(), ()>("bar", ())
    //             }

    //             fn goop(&mut self, flag: bool) -> u32 {
    //                 self.e.was_called_returning::<bool, u32>("goop", flag)
    //             }

    //             fn zing(&self, first: i32, second: bool) {
    //                 self.e.was_called::<(i32, bool), ()>("zing", (first, second))
    //             }
    //         }
    //     };

    //     let result = simulacrum_internal(input.as_str());

    //     assert_eq!(expected, result);
    // }
}

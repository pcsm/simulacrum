#![feature(proc_macro)]

             extern crate syn;
#[macro_use] extern crate quote;
             extern crate proc_macro;

use proc_macro::TokenStream;
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

        pub struct #name;
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
            pub trait ContainerBehavior<D, E> {
                // Retrieve a list of this views's children, if you have any.
                fn children(&self) -> Option<&Vec<ViewRef<E>>>;

                /// The view's frame has been updated, so lay out its children.
                fn layout(&mut self, state: &ViewState<D>);

                /// This view's data has been updated, so lay out its children if you need to.
                #[allow(unused_variables)]
                fn data_updated(&mut self, state: &ViewState<D>) { }
                
                // Lifecycle
                #[allow(unused_variables)]
                fn setup(&mut self, state: &ViewState<D>) { }
                #[allow(unused_variables)]
                fn cleanup(&mut self, state: &ViewState<D>) { }
            }
        };

        let expected = quote! {
            pub trait ContainerBehavior<D, E> {
                // Retrieve a list of this views's children, if you have any.
                fn children(&self) -> Option<&Vec<ViewRef<E> > >;

                /// The view's frame has been updated, so lay out its children.
                fn layout(&mut self, state: &ViewState<D>);

                /// This view's data has been updated, so lay out its children if you need to.
                #[allow(unused_variables)]
                fn data_updated(&mut self, state: &ViewState<D>) { }
                
                // Lifecycle
                #[allow(unused_variables)]
                fn setup(&mut self, state: &ViewState<D>) { }
                #[allow(unused_variables)]
                fn cleanup(&mut self, state: &ViewState<D>) { }
            }

            pub struct ContainerBehaviorMock;
        };

        let result = simulacrum_internal(input.as_str());

        assert_eq!(result, expected);
    }
}

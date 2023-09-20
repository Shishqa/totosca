use proc_macro::{self, TokenStream};
use quote::quote;

#[macro_use]
extern crate syn;

use syn::{Data, DeriveInput, Fields, Type};

#[proc_macro_derive(Parse, attributes(parse))]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;


}


#[proc_macro_derive(FromValue)]
pub fn from_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut field_impls = Vec::new();
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let ident = field.ident.as_ref().unwrap();
                    let stype = match &field.ty {
                        Type::Path(v) => v,
                        _ => unimplemented!("Deriving FromValue not possible for field: {}", ident),
                    };
                    field_impls.push(
                        // FIXME 1: replace unwrap with a nice error message about a missing field
                        // FIXME 2: replace ? with nice error message about wrong type
                        // FIXME 3: this does not work for types like Option<T> and HashMap<K, V>
                        //          where only Option::from_value and HashMap::from_value should be
                        //          used, but I can't find a way to strip generics from the path
                        quote! { #ident: #stype::from_value(map.get("#ident").unwrap())?,
                        },
                    );
                }
            },
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    let mut fields = proc_macro2::TokenStream::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics Parse for #name #ty_generics #where_clause {
            fn from_yaml(n: &yaml_peg::NodeRc) -> Result<Self, Vec<Error>> {
                use ::std::option::Option;
                use ::std::vec::Vec;

                let mut errors = Vec::<Error>::new();

                #field

                let map: HashMap<String, Value> = HashMap::from_value(value)?;

                Ok(#name {
                    #fields
                })
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}

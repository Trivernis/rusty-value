use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, DataStruct, DeriveInput, FieldsNamed, FieldsUnnamed, Generics,
    WhereClause, WherePredicate,
};

#[proc_macro_derive(RustyValue)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    derive(parse_macro_input!(input as DeriveInput))
}

fn derive(input: DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Struct(s) => derive_struct(&input, s),
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => panic!("unions are currently unsupported"),
    }
}

fn derive_struct(input: &DeriveInput, struct_data: &DataStruct) -> TokenStream {
    let ident = &input.ident;
    let name = ident.to_string();
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let where_clause = add_rusty_bound(&input.generics);

    match &struct_data.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            let field_idents = named.iter().map(|f| f.ident.as_ref()).collect::<Vec<_>>();
            let field_names = named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>();
            let field_count = named.len();

            TokenStream::from(quote! {
                impl #impl_generics rusty_value::RustyValue for #ident #ty_generics #where_clause {
                    fn into_rusty_value(self) -> rusty_value::Value {
                        use rusty_value::*;
                        let mut values = std::collections::HashMap::with_capacity(#field_count);

                        #(
                            values.insert(#field_names.to_string(), self.#field_idents.into_rusty_value());
                        )*

                        Value::Struct(Struct{
                            name: #name.to_string(),
                            fields: StructFields::Named(values),
                        })
                    }
                }
            })
        }
        syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let field_indices = unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| syn::Index::from(i))
                .collect::<Vec<_>>();
            let field_count = unnamed.len();

            TokenStream::from(quote! {
                impl #impl_generics rusty_value::RustyValue for #ident #ty_generics #where_clause {
                    fn into_rusty_value(self) -> rusty_value::Value {
                        use rusty_value::*;
                        let mut values = Vec::with_capacity(#field_count);

                        #(
                            values.push(self.#field_indices.into_rusty_value());
                        )*

                        Value::Struct(Struct{
                            name: #name.to_string(),
                            fields: StructFields::Unnamed(values),
                        })
                    }
                }
            })
        }
        syn::Fields::Unit => TokenStream::from(quote! {
                impl #impl_generics rusty_value::RustyValue for #ident #ty_generics #where_clause {
                    fn into_rusty_value(self) -> rusty_value::Value {
                        Value::Unit(#name.to_string())
                    }
                }
        }),
    }
}

fn add_rusty_bound(generics: &Generics) -> WhereClause {
    let trait_bound: proc_macro2::TokenStream = parse_quote!(rusty_value::RustyValue);

    let new_predicates = generics.type_params().map::<WherePredicate, _>(|param| {
        let param = &param.ident;
        parse_quote!(#param : #trait_bound)
    });

    let mut generics = generics.clone();
    generics
        .make_where_clause()
        .predicates
        .extend(new_predicates);
    generics.where_clause.unwrap()
}

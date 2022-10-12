use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, DataEnum, DataStruct, DeriveInput, FieldsNamed, FieldsUnnamed,
    Generics, Variant, WhereClause, WherePredicate,
};

#[proc_macro_derive(RustyValue)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    derive(parse_macro_input!(input as DeriveInput))
}

fn derive(input: DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Struct(s) => derive_struct(&input, s),
        syn::Data::Enum(e) => derive_enum(&input, e),
        syn::Data::Union(_) => panic!("unions are currently unsupported"),
    }
}

fn derive_struct(input: &DeriveInput, struct_data: &DataStruct) -> TokenStream {
    let ident = &input.ident;
    let name = ident.to_string();
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let where_clause = add_rusty_bound(&input.generics);
    let rusty_value = get_rusty_value_crate();

    match &struct_data.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            let field_idents = named.iter().map(|f| f.ident.as_ref()).collect::<Vec<_>>();
            let field_names = named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>();
            let field_count = named.len();

            TokenStream::from(quote! {
                impl #impl_generics #rusty_value::RustyValue for #ident #ty_generics #where_clause {
                    fn into_rusty_value(self) -> #rusty_value::Value {
                        use #rusty_value::*;
                        let mut values = std::collections::HashMap::with_capacity(#field_count);

                        #(
                            values.insert(#field_names.to_string(), self.#field_idents.into_rusty_value());
                        )*

                        Value::Struct(Struct{
                            name: #name.to_string(),
                            fields: Fields::Named(values),
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
                impl #impl_generics #rusty_value::RustyValue for #ident #ty_generics #where_clause {
                    fn into_rusty_value(self) -> #rusty_value::Value {
                        use #rusty_value::*;
                        let mut values = Vec::with_capacity(#field_count);

                        #(
                            values.push(self.#field_indices.into_rusty_value());
                        )*

                        Value::Struct(Struct{
                            name: #name.to_string(),
                            fields: Fields::Unnamed(values),
                        })
                    }
                }
            })
        }
        syn::Fields::Unit => TokenStream::from(quote! {
                impl #impl_generics #rusty_value::RustyValue for #ident #ty_generics #where_clause {
                    fn into_rusty_value(self) -> #rusty_value::Value {
                        use #rusty_value::*;
                        Value::Struct(Struct{
                            name: #name.to_string(),
                            fields: Fields::Unit,
                        })
                    }
                }
        }),
    }
}

fn derive_enum(input: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let where_clause = add_rusty_bound(&input.generics);
    let variant_matchers = enum_data
        .variants
        .iter()
        .map(|v| create_enum_value_match(ident, v))
        .collect::<Vec<_>>();
    let rusty_value = get_rusty_value_crate();

    TokenStream::from(quote! {
        impl #impl_generics #rusty_value::RustyValue for #ident #ty_generics #where_clause {
            fn into_rusty_value(self) -> #rusty_value::Value {
                let enum_val = match self {
                    #( #variant_matchers )*
                };
                #rusty_value::Value::Enum(enum_val)
            }
        }
    })
}

fn create_enum_value_match(ident: &syn::Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let enum_name = ident.to_string();
    let variant_ident = &variant.ident;
    let variant_name = variant_ident.to_string();
    let rusty_value = get_rusty_value_crate();

    match &variant.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            let field_idents = named.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let field_names = named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>();
            let field_count = named.len();

            quote! {
                #ident::#variant_ident { #( #field_idents, )* } => {
                    use #rusty_value::*;

                    let mut fields = std::collections::HashMap::with_capacity(#field_count);
                    #(
                        fields.insert(#field_names.to_string(), #field_idents.into_rusty_value());
                    )*
                    Enum {
                        name: #enum_name.to_string(),
                        variant: #variant_name.to_string(),
                        fields: Fields::Named(fields)
                    }
                }
            }
        }
        syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let field_names = unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| syn::Ident::new(&format!("f{i}"), Span::call_site()))
                .collect::<Vec<_>>();
            let field_count = unnamed.len();

            quote! {
                #ident::#variant_ident ( #( #field_names, )* ) => {
                    use #rusty_value::*;

                    let mut fields = Vec::with_capacity(#field_count);
                    #(
                        fields.push(#field_names.into_rusty_value());
                    )*
                    Enum {
                        name: #enum_name.to_string(),
                        variant: #variant_name.to_string(),
                        fields: Fields::Unnamed(fields)
                    }
                }
            }
        }
        syn::Fields::Unit => quote! {
            #ident::#variant_ident => {
                use #rusty_value::*;

                Enum {
                    name: #enum_name.to_string(),
                    variant: #variant_name.to_string(),
                    fields: Fields::Unit
                }
            }
        },
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

fn get_rusty_value_crate() -> proc_macro2::TokenStream {
    use proc_macro_crate::{crate_name, FoundCrate};
    match crate_name("rusty_value") {
        Ok(FoundCrate::Itself) => quote!(rusty_value),
        Err(_) => quote!(crate),
        Ok(FoundCrate::Name(name)) => quote!(#name),
    }
}

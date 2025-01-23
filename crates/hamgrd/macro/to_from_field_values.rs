use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    Meta, MetaNameValue,
};

pub fn to_from_field_values(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named: fields, .. }),
        ..
    }) = input.data
    else {
        panic!("ToFromFieldValues can only be derived on a struct with named fields")
    };

    let name = input.ident;
    let to_field_values = to_field_values(&fields);
    let from_field_values = from_field_values(&fields);

    let output = quote! {
        #[automatically_derived]
        impl crate::tables::support::ToFromFieldValues for #name {
            #to_field_values
            #from_field_values
        }
    };

    output.into()
}

/// Generate the implementation of `to_field_values`.
pub fn to_field_values(fields: &Punctuated<Field, Comma>) -> TokenStream {
    /// Generate one field
    fn field(field: &Field) -> TokenStream {
        let name = &field.ident;
        let fv_name = fv_name(field);
        quote! {
            fvs.insert(String::from(#fv_name), self.#name.to_field_value());
        }
    }

    let fields = fields.iter().map(field).collect::<Vec<_>>();

    quote! {
        fn to_field_values(&self) -> swss_common::FieldValues {
            use crate::tables::support::ToFromFieldValue;
            let mut fvs = ::std::collections::HashMap::new();
            #(#fields)*
            fvs
        }
    }
}

/// Generate the implementation of `from_field_values`.
fn from_field_values(fields: &Punctuated<Field, Comma>) -> TokenStream {
    /// Generate one field assignment: `field: parse_field_value(fvs, "field")?`
    fn field(field: &Field) -> TokenStream {
        let name = &field.ident;
        let fv_name = fv_name(field);
        quote! {
            #name: crate::tables::support::parse_field_value(#fv_name, fvs)?
        }
    }

    let fields = fields.iter().map(field).collect::<Vec<_>>();

    quote! {
        fn from_field_values(fvs: &swss_common::FieldValues) -> Result<Self, crate::tables::support::FromFieldValuesError> {
           Ok(Self {
               #(#fields),*
           })
        }
    }
}

/// Extract the desired field name from a struct field
fn fv_name(field: &Field) -> TokenStream {
    // If we have #[rename = "x"] return "x"
    for attr in &field.attrs {
        match &attr.meta {
            Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("rename") => return quote! { #value },
            _ => (),
        }
    }

    // Else just stringify the field name
    let name = &field.ident;
    quote! { ::std::stringify!(#name) }
}

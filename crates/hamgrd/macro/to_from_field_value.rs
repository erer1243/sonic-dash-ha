use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Fields, Ident, LitStr, Variant};

pub fn to_from_field_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(data) = input.data else {
        panic!("ToFromFieldValue can only be derived on enums")
    };
    assert!(
        data.variants.iter().all(|v| v.fields == Fields::Unit),
        "ToFromFieldValue only supports unit variants (no fields)"
    );

    let name = &input.ident;
    let to_field_value = to_field_value(&data.variants);
    let from_field_value = from_field_value(&data.variants);

    let output = quote! {
        #[automatically_derived]
        impl crate::tables::support::ToFromFieldValue for #name {
            #to_field_value
            #from_field_value
        }
    };

    output.into()
}

fn to_field_value(variants: &Punctuated<Variant, Comma>) -> TokenStream {
    fn match_arm(name: &Ident) -> TokenStream {
        let str_value = ident_as_lowercase_litstr(name);
        quote! {
            Self::#name => #str_value.into(),
        }
    }

    let match_arms = variants.iter().map(|v| match_arm(&v.ident)).collect::<Vec<_>>();

    quote! {
        fn to_field_value(&self) -> swss_common::CxxString {
            match self {
                #(#match_arms)*
            }
        }
    }
}

fn from_field_value(variants: &Punctuated<Variant, Comma>) -> TokenStream {
    fn match_arm(name: &Ident) -> TokenStream {
        let match_value = ident_as_lowercase_litstr(name);
        quote! {
            #match_value => Ok(Self::#name),
        }
    }

    fn valid_variants(variants: &Punctuated<Variant, Comma>) -> TokenStream {
        let lowercase_litstrs = variants
            .iter()
            .map(|v| ident_as_lowercase_litstr(&v.ident))
            .collect::<Vec<_>>();
        quote! { &[ #(#lowercase_litstrs),* ] }
    }

    let match_arms = variants.iter().map(|v| match_arm(&v.ident)).collect::<Vec<_>>();
    let valid_variants = valid_variants(&variants);

    quote! {
        fn from_field_value(value: Option<&[u8]>) -> Result<Self, crate::tables::support::FromFieldValueError> {
            use crate::tables::support::{FromFieldValueError, InvalidVariantError};

            let Some(bytes) = value else { return Err(FromFieldValueError::Missing) };
            let mut s = String::from_utf8_lossy(bytes).into_owned();
            s.make_ascii_lowercase();

            match &*s {
                #(#match_arms)*
                _ => Err(FromFieldValueError::Invalid {
                    data: s,
                    error: Box::new(InvalidVariantError {
                        valid_variants: #valid_variants
                    }),
                }),
            }
        }
    }
}

fn ident_as_lowercase_litstr(i: &Ident) -> LitStr {
    LitStr::new(&i.to_string().to_lowercase(), Span::call_site())
}

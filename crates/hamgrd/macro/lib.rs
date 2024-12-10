mod to_from_field_value;
mod to_from_field_values;

/// Implement `ToFromFieldValues` on a struct.
#[proc_macro_derive(ToFromFieldValues, attributes(rename))]
pub fn to_from_field_values(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    to_from_field_values::to_from_field_values(input)
}

/// Implement `ToFromFieldValue` on an enum
#[proc_macro_derive(ToFromFieldValue)]
pub fn to_from_field_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    to_from_field_value::to_from_field_value(input)
}

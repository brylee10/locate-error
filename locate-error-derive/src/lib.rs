extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Generics, Ident, Type,
    parse_macro_input, parse_quote, spanned::Spanned,
};

/// This macro is used to implement `From` on an enum or struct and locating
/// where the `From` impl is called. Typically used for tracking sources of bubbling errors with `thiserror`.
#[proc_macro_derive(Locate, attributes(locate_from))]
pub fn locate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let generics = &input.generics;

    let from_attributes: Vec<Attribute> = parse_quote!(
        #[allow(
            deprecated,
            unused_qualifications,
            clippy::elidable_lifetime_names,
            clippy::needless_lifetimes,
        )]
        #[automatically_derived]
    );

    match &input.data {
        Data::Enum(data) => process_enum(data, &from_attributes, generics, ident),
        Data::Struct(data) => process_struct(data, &from_attributes, generics, ident),
        _ => TokenStream::from(quote! {
            compile_error!("Locate can only be derived for enums or structs");
        }),
    }
}

fn process_enum(
    data: &DataEnum,
    from_attributes: &[Attribute],
    generics: &Generics,
    ident: &Ident,
) -> TokenStream {
    let mut from_impls = vec![];
    let mut n_has_locate_from = 0;
    for variant in &data.variants {
        let variant_name = &variant.ident;
        let fields = &variant.fields;

        match &fields {
            Fields::Unnamed(fields) => {
                for field in fields.unnamed.iter() {
                    if let Some(index) = locate_from_attr_index(&field.attrs) {
                        if fields.unnamed.len() != 2 {
                            return TokenStream::from(quote_spanned! {
                                variant.ident.span() => compile_error!("Locate requires enums variants with the #[locate_from] attribute to have exactly two fields, one for the source and one for the location");
                            });
                        }
                        if let Some(other_field) = fields.unnamed.iter().nth((index + 1) % 2) {
                            if !is_location_type(&other_field.ty) {
                                return TokenStream::from(quote_spanned! {
                                    other_field.ident.span() => compile_error!("Variants with #[locate_from] must have a field of type `locate_from::Location`");
                                });
                            }
                        }
                        n_has_locate_from += 1;
                        if let Type::Path(path) = &field.ty {
                            let field_type = &path.path;
                            from_impls.push(quote! {
                                #(#from_attributes)*
                                impl #generics ::core::convert::From<#field_type> for #ident #generics {
                                    #[track_caller]
                                    fn from(value: #field_type) -> Self {
                                        let location = ::std::panic::Location::caller();
                                        #ident::#variant_name {
                                            0: value,
                                            1: ::locate_error::Location {
                                                file: location.file().to_string(),
                                                line: location.line(),
                                                column: location.column(),
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
            Fields::Named(fields) => {
                for field in fields.named.iter() {
                    // Field name will be present for named fields
                    let field_name = field.ident.as_ref().unwrap();
                    if locate_from_attr_index(&field.attrs).is_some() {
                        let has_location_field = fields.named.iter().any(|f| {
                            f.ident.as_ref().is_some_and(|name| name == "location")
                                && is_location_type(&f.ty)
                        });

                        if !has_location_field {
                            return TokenStream::from(quote_spanned! {
                                variant.ident.span() => compile_error!("Variants with #[locate_from] must have a field named 'location' of type `locate_from::Location`");
                            });
                        }

                        if fields.named.len() != 2 {
                            return TokenStream::from(quote_spanned! {
                                variant.ident.span() => compile_error!("Locate requires enums variants with the #[locate_from] attribute to have exactly two fields, one for the source and one for the location");
                            });
                        }

                        n_has_locate_from += 1;
                        if let Type::Path(path) = &field.ty {
                            let field_type = &path.path;
                            from_impls.push(quote! {
                                #(#from_attributes)*
                                impl #generics ::core::convert::From<#field_type> for #ident #generics {
                                    #[track_caller]
                                    fn from(value: #field_type) -> Self {
                                        let location = ::std::panic::Location::caller();
                                        #ident::#variant_name {
                                            #field_name:value,
                                            location: ::locate_error::Location {
                                                file: location.file().to_string(),
                                                line: location.line(),
                                                column: location.column(),
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
            Fields::Unit => {}
        }
    }

    if n_has_locate_from == 0 {
        return TokenStream::from(quote! {
            compile_error!("Locate requires at least one variant with the #[locate_from] attribute (otherwise this macro is effectively a no-op)");
        });
    }

    let expanded = quote! {
        #(#from_impls)*
    };

    TokenStream::from(expanded)
}

fn process_struct(
    data: &DataStruct,
    from_attributes: &[Attribute],
    generics: &Generics,
    ident: &Ident,
) -> TokenStream {
    let mut from_impl: proc_macro2::TokenStream = quote! {};
    // Find fields with locate_from attribute
    let locate_from_fields: Vec<_> = data
        .fields
        .iter()
        .filter(|field| locate_from_attr_index(&field.attrs).is_some())
        .collect();

    // Check if there's exactly one field with locate_from
    if locate_from_fields.is_empty() || locate_from_fields.len() > 1 {
        let error_message = format!(
            "Locate requires exactly one field marked with #[locate_from], found {:?}",
            locate_from_fields.len()
        );
        return TokenStream::from(quote! {
            compile_error!(#error_message);
        });
    }

    // There can be at most 2 fields (one with the locate_from attribute and one with the location field)
    if data.fields.len() > 2 {
        return TokenStream::from(quote! {
            compile_error!("Locate requires structs to have only a 'source' field (with the #[locate_from] attribute) and a 'location' field");
        });
    }

    // Check if there's a field named "location"
    let has_location_field = data
        .fields
        .iter()
        .any(|field| field.ident.as_ref().is_some_and(|name| name == "location"));

    if !has_location_field {
        return TokenStream::from(quote! {
            compile_error!("Locate requires structs to have a field named 'location' of type `locate_from::Location`");
        });
    }

    let field = locate_from_fields.first().unwrap();
    let field_name = field.ident.as_ref().unwrap();

    if let Type::Path(path) = &field.ty {
        let field_type = &path.path;
        from_impl = quote! {
            #(#from_attributes)*
            impl #generics ::core::convert::From<#field_type> for #ident #generics {
                #[track_caller]
                fn from(value: #field_type) -> Self {
                    let location = ::std::panic::Location::caller();
                    #ident {
                        #field_name: value,
                        location: ::locate_error::Location {
                            file: location.file().to_string(),
                            line: location.line(),
                            column: location.column(),
                        }
                    }
                }
            }
        };
    }

    TokenStream::from(from_impl)
}

fn locate_from_attr_index(attributes: &[Attribute]) -> Option<usize> {
    attributes.iter().position(|attr| {
        if !attr.path().is_ident("locate_from") {
            return false;
        }
        // Only allow #[locate_from], not #[locate_from = "some_path"]
        matches!(attr.meta, syn::Meta::Path(_))
    })
}

// Helper function to check if a type is Location (may not identify full path correctly, but works in most cases)
fn is_location_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(last_segment) = type_path.path.segments.last() {
            // Check if the last segment is "Location"
            if last_segment.ident == "Location" {
                // Simplistic check, verify the last segment
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_locate_from_attr_index() {
        let attributes = vec![];
        assert!(locate_from_attr_index(&attributes).is_none());

        // Test with the locate_from attribute
        let locate_from_attr: Attribute = parse_quote!(#[locate_from]);
        let attributes = vec![locate_from_attr];
        assert!(locate_from_attr_index(&attributes) == Some(0));

        // Test does not have the locate_from attribute
        let locate_from_attr: Attribute = parse_quote!(#[locate_from = "some_path"]);
        let attributes = vec![locate_from_attr];
        assert!(locate_from_attr_index(&attributes).is_none());

        // Test with multiple attributes including locate_from
        let other_attr: Attribute = parse_quote!(#[derive(Debug)]);
        let locate_from_attr: Attribute = parse_quote!(#[locate_from]);
        let attributes = vec![other_attr, locate_from_attr];
        assert!(locate_from_attr_index(&attributes).is_some());

        // Test with multiple attributes but without locate_from
        let attr1: Attribute = parse_quote!(#[derive(Debug)]);
        let attr2: Attribute = parse_quote!(#[derive(Clone)]);
        let attributes = vec![attr1, attr2];
        assert!(locate_from_attr_index(&attributes).is_none());
    }
}

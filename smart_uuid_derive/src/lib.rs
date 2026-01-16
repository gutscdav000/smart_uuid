use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro for implementing the `UuidType` trait.
///
/// This macro automatically generates:
/// - `discriminant()` - returns a unique byte for each variant (0, 1, 2, ...)
/// - `from_discriminant()` - reconstructs the variant from a byte
/// - `prefix()` - returns a snake_case string prefix for the variant
///
/// # Example
/// ```ignore
/// #[derive(UuidType)]
/// enum UserType {
///     Retail,                      // discriminant=0, prefix="retail"
///     Business,                    // discriminant=1, prefix="business"
///     #[uuid_type(prefix = "org")] // override prefix
///     Organization,                // discriminant=2, prefix="org"
/// }
/// ```
#[proc_macro_derive(UuidType, attributes(uuid_type))]
pub fn derive_uuid_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = impl_uuid_type(&input);

    TokenStream::from(expanded)
}

fn impl_uuid_type(input: &DeriveInput) -> TokenStream2 {
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(input, "UuidType can only be derived for enums")
                .to_compile_error();
        }
    };

    // Check that all variants are unit variants (no fields)
    for variant in variants.iter() {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(
                variant,
                "UuidType can only be derived for enums with unit variants (no fields)",
            )
            .to_compile_error();
        }
    }

    // Check we don't have more than 256 variants
    if variants.len() > 256 {
        return syn::Error::new_spanned(
            input,
            "UuidType can only be derived for enums with at most 256 variants",
        )
        .to_compile_error();
    }

    // Generate discriminant match arms
    let discriminant_arms: Vec<_> = variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let variant_name = &v.ident;
            let discriminant = i as u8;
            quote! { Self::#variant_name => #discriminant }
        })
        .collect();

    // Generate from_discriminant match arms
    let from_discriminant_arms: Vec<_> = variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let variant_name = &v.ident;
            let discriminant = i as u8;
            quote! { #discriminant => ::core::option::Option::Some(Self::#variant_name) }
        })
        .collect();

    // Generate prefix match arms
    let prefix_arms: Vec<_> = variants
        .iter()
        .map(|v| {
            let variant_name = &v.ident;
            let prefix = get_prefix_from_attrs(&v.attrs)
                .unwrap_or_else(|| to_snake_case(&variant_name.to_string()));
            quote! { Self::#variant_name => #prefix }
        })
        .collect();

    quote! {
        impl smart_uuid::UuidType for #name {
            fn discriminant(&self) -> u8 {
                match self {
                    #(#discriminant_arms,)*
                }
            }

            fn from_discriminant(value: u8) -> ::core::option::Option<Self> {
                match value {
                    #(#from_discriminant_arms,)*
                    _ => ::core::option::Option::None,
                }
            }

            fn prefix(&self) -> &'static str {
                match self {
                    #(#prefix_arms,)*
                }
            }
        }
    }
}

/// Extract custom prefix from #[uuid_type(prefix = "...")] attribute
fn get_prefix_from_attrs(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("uuid_type") {
            continue;
        }

        // Parse #[uuid_type(prefix = "...")]
        let mut prefix = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("prefix") {
                let value: syn::LitStr = meta.value()?.parse()?;
                prefix = Some(value.value());
            }
            Ok(())
        });

        if prefix.is_some() {
            return prefix;
        }
    }
    None
}

/// Convert PascalCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Mono, attributes(base))]
pub fn derive_mono(input: TokenStream) -> TokenStream {
    // Parse the input into a Rust syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;

    // Extract the generic parameters and the `where` clause
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Locate the field marked with `#[base]` and extract its identifier and type
    let mut base_field = None;
    if let syn::Data::Struct(data_struct) = &input.data {
        for field in &data_struct.fields {
            if field.attrs.iter().any(|attr| attr.path().is_ident("base")) {
                base_field = Some(field);
                break;
            }
        }
    }

    let base_field = match base_field {
        Some(field) => field,
        None => panic!("#[base] attribute not found in struct"),
    };

    let base_field_ident = base_field.ident.as_ref().expect("Expected named field");
    let base_field_type = &base_field.ty;

    // Generate the `Deref`, `DerefMut`, and `PartialEq` implementations with generic support
    let expanded = quote! {
        // Implement Deref
        impl #impl_generics std::ops::Deref for #struct_name #ty_generics #where_clause {
            type Target = #base_field_type;

            fn deref(&self) -> &Self::Target {
                &self.#base_field_ident
            }
        }

        // Implement DerefMut
        impl #impl_generics std::ops::DerefMut for #struct_name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#base_field_ident
            }
        }

        // Implement PartialEq
        impl #impl_generics PartialEq for #struct_name #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                std::ops::Deref::deref(self).eq(other)
            }
        }
    };

    // Convert expanded code back into a TokenStream and return it
    TokenStream::from(expanded)
}

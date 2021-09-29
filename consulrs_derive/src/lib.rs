#[macro_use]
extern crate synstructure;
extern crate proc_macro;

mod error;

use error::Error;
use proc_macro2::Span;

const FIELD_NAME: &str = "features";

/// Returns field names of the given struct.
fn fields(data: &syn::DataStruct) -> Vec<String> {
    data.fields
        .iter()
        .map(|f| f.ident.clone().unwrap().to_string())
        .collect()
}

fn endpoint_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    // Validate the required field exists
    if let syn::Data::Struct(data) = &s.ast().data {
        let fields = fields(data);
        if !fields.contains(&FIELD_NAME.into()) {
            return Error::new(data.struct_token.span, "Missing required `features` field")
                .into_tokens();
        }
    } else {
        return Error::new(Span::call_site(), "May only be used on with structs").into_tokens();
    }

    s.gen_impl(quote! {
        use crate::api::features::{FeaturedEndpoint, Features};

        gen impl FeaturedEndpoint for @Self {
            fn features(&self) -> Option<Features> {
                self.features.clone()
            }
        }
    })
}

synstructure::decl_derive!([QueryEndpoint] => endpoint_derive);

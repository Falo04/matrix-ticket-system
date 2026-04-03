use darling::FromAttributes;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;
use syn::DeriveInput;
use syn::Type;
use syn::parse2;

pub fn derive_bm_uuid_impl(input: TokenStream) -> TokenStream {
    match parse_bm_uuid(input) {
        Ok(parsed) => generate_bm_uuid(&parsed),
        Err(err) => err.write_errors(),
    }
}

/// Internal representation of the parsed input for the `ModelId` derive macro.
pub struct ParsedBusinessModelUuid {
    pub ident: Ident,
    pub data: Data,
    pub annos: BusinessModelUuidAnnotations,
}

/// Supported attributes for `#[model_id(...)]`.
#[derive(FromAttributes, Debug)]
#[darling(attributes(bm_uuid))]
pub struct BusinessModelUuidAnnotations {
    /// The `rorm::Model` type this identifier associated with.
    pub model: Type,
}

pub fn parse_bm_uuid(input: TokenStream) -> darling::Result<ParsedBusinessModelUuid> {
    let input: DeriveInput = parse2(input)?;
    let mut errors = darling::Error::accumulator();
    let annos = errors.handle(BusinessModelUuidAnnotations::from_attributes(&input.attrs));
    errors.finish()?;
    Ok(ParsedBusinessModelUuid {
        ident: input.ident,
        data: input.data,
        annos: annos.unwrap(),
    })
}

pub fn generate_bm_uuid(input: &ParsedBusinessModelUuid) -> TokenStream {
    let name = &input.ident;
    let model = &input.annos.model;

    match &input.data {
        Data::Struct(data)
            // Validates that the struct is a tuple struct with exactly one field: e.g., StructName(Uuid)
            if data.fields.len() == 1 && data.fields.iter().next().unwrap().ident.is_none() =>
        {
            quote! {
                impl BusinessModelUuid<#model> for #name {
                    fn new_from_model(value: galvyn::rorm::prelude::ForeignModel<#model>) -> Self {
                        Self(value.0)
                    }

                    fn get_inner(&self) -> Uuid {
                        self.0
                    }
                }
            }
        }
        _ => {
            quote! { compile_error!("BmUuid derive currently only supports single-field tuple structs"); }
        }
    }
}

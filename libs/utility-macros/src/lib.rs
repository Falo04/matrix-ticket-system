mod bm_uuid;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(BusinessModelUuid, attributes(bm_uuid))]
pub fn derive_business_model_uuid(input: TokenStream) -> TokenStream {
    bm_uuid::derive_bm_uuid_impl(input.into()).into()
}

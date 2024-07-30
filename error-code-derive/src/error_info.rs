use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_error_info(_input: DeriveInput) -> TokenStream {
    quote! {}
}

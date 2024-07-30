mod error_info;

use error_info::process_error_info;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    process_error_info(input).into()
}

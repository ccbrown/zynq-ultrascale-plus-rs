use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let name_string = name.to_string();
    let body = &input.block;
    let attrs = &input.attrs;

    quote! {
        #[test_case]
        #(#attrs)*
        fn #name() #ret {
            debug!("test {}::{} ...", module_path!().strip_prefix("zynq_ultrascale_plus::").unwrap(), #name_string);

            #body
        }
    }
    .into()
}

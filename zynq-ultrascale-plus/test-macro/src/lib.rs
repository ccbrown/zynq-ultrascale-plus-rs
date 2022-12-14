use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let name_string = name.to_string();
    let body = &input.block;
    let attrs = &input.attrs;

    let qemu_only = args.iter().any(|a| match a {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => path
            .get_ident()
            .map(|id| id == "qemu_only")
            .unwrap_or(false),
        _ => false,
    });

    if qemu_only {
        quote! {
            #[test_case]
            #(#attrs)*
            fn #name() #ret {
                if !crate::tests::is_qemu() {
                    debug!("test {}::{} ... skipping (qemu only)", module_path!().strip_prefix("zynq_ultrascale_plus::").unwrap(), #name_string);
                    return;
                }

                debug!("test {}::{} ...", module_path!().strip_prefix("zynq_ultrascale_plus::").unwrap(), #name_string);

                #body
            }
        }
        .into()
    } else {
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
}

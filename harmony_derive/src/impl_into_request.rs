use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, Ident, Lit, NestedMeta};

pub(crate) fn impl_into_request(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let args = parse_macro_input!(args as AttributeArgs);

    let req_types = get_req_types(args);
    let impls = req_types
        .into_iter()
        .map(|req_type| impl_into_req(&input, req_type))
        .collect::<Vec<_>>();

    (quote! {
        #input
        #(#impls)*
    })
    .into()
}

fn get_req_types(args: Vec<NestedMeta>) -> Vec<Ident> {
    args.into_iter()
        .map(|arg| match arg {
            NestedMeta::Lit(lit) => {
                let span = lit.span();
                let lit = match lit {
                    Lit::Verbatim(a) => a.to_string(),
                    Lit::Str(a) => a.value(),
                    _ => panic!("Verbatim expected"),
                };
                Ident::new(&lit, span)
            }
            NestedMeta::Meta(_) => panic!("Literal expected"),
        })
        .collect()
}

fn impl_into_req(input: &DeriveInput, req: Ident) -> TokenStream2 {
    let fields = match &input.data {
        Data::Struct(s) => s
            .fields
            .iter()
            .map(|field| field.ident.as_ref().expect("No ident")),
        _ => panic!("Not a struct"),
    };
    let name = &input.ident;
    quote! {
        impl ::std::convert::From<#name> for #req {
            fn from(o: #name) -> Self {
                #req {
                    #(#fields: o.#fields.into(),)*
                }
            }
        }
    }
}

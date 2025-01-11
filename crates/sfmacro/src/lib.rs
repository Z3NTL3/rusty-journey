use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, ItemStruct, MetaNameValue, Token};

struct Attrs {
    args: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]>,
}

impl syn::parse::Parse for Attrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Attrs{
            args: input.parse_terminated(MetaNameValue::parse, Token![,])?
        })
    }
}

#[proc_macro_attribute]
pub fn scrape_website(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse::<Attrs>(args).unwrap();
    let mut url = String::default();

    for arg in attr.args {
        if arg.path.is_ident("url") {
            url = arg.value.to_token_stream().to_string();
        }
    }

    url = url.replace("\"", "");
    let ItemStruct{
        ident,
        generics,
        fields,
        ..
    } = syn::parse::<ItemStruct>(item).unwrap();

    let fields_iter = fields.iter().map(|field| field);
    quote! {
        #[derive(std::default::Default)]
        struct #ident #generics {
            #(#fields_iter),*
        }

        impl #ident #generics {
            pub async fn scrape(&self) -> Result<String, reqwest::Error> {
                let body = reqwest::get(#url)
                    .await?
                    .text()
                    .await?;
                Ok(body.into())
            }
        }
         
    }.into()
}
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

 
    let mut fields_iter = Vec::<syn::Field>::default();
    for field in fields {
        fields_iter.push(field);
    }
    quote! {
        struct #ident #generics {
            page_content: String,
            #(#fields_iter),*
        }

        impl #ident #generics {
            pub async fn scrape(&mut self) -> Result<(), reqwest::Error> {
                let body = reqwest::get(#url)
                    .await?
                    .text()
                    .await?;
                self.page_content = body.into();
                Ok(())
            }
        }

        impl std::default::Default for #ident #generics {
            fn default() -> Self {
                Self { page_content: Default::default(), title: Default::default() }
            }
        }
        
    }.into()
}




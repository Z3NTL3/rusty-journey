use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, Attribute, ItemStruct, Meta, MetaNameValue, Token};

#[cfg(test)]
mod test {
    use quote::quote;

    #[test]
    fn test_macro(){
        use super::scrape_website_page_impl;

        scrape_website_page_impl(
        quote! {
                scrape_website_page(url="test")
            }, 
        quote! {
                #[scrape_website_page(url="test")]
                struct Page {
                    title: String
                }
            }
        );
    }
}

struct Attrs {
    ident: syn::Ident,
    args: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]>,
    parentheses: syn::token::Paren
}

impl syn::parse::Parse for Attrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Attrs{
            ident: input.parse()?,
            parentheses: syn::parenthesized!(content in input),
            args: content.parse_terminated(MetaNameValue::parse, Token![,])?
        })
    }
}

#[proc_macro_attribute]
pub fn scrape_website_page(args: TokenStream, item: TokenStream) -> TokenStream {
    scrape_website_page_impl(args.into(), item.into()).into()
}

fn scrape_website_page_impl(args: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let attr = syn::parse2::<Attrs>(args).unwrap();

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
    } = syn::parse2::<ItemStruct>(item).unwrap();

    quote! {
        struct #ident #generics {
            page_content: String,
            url: String,
            #fields
        }

        impl #ident #generics {
            // just as demonstration
            pub fn scrape(&mut self) -> String {
                self.url = #url;
                String::from(#url)
            }
        }
    }

}

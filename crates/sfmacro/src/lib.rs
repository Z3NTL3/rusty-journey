use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, Attribute};

#[cfg(test)]
mod test {
    use quote::quote;

    #[test]
    fn test_macro(){
        use super::scrape_website_page_impl;

        scrape_website_page_impl(
        quote! {
                #[scrape_website_page(url="test")]
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

#[proc_macro_attribute]
pub fn scrape_website_page(args: TokenStream, item: TokenStream) -> TokenStream {
    scrape_website_page_impl(args.into(), item.into()).into()
}

fn scrape_website_page_impl(args: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let args = syn::parse2::<AttrArgs>(args).unwrap();
    let syn::ItemStruct{
        ident,
        generics,
        fields,
        ..
    } = syn::parse2::<syn::ItemStruct>(item).unwrap();

    for attr in args.attrs {
        attr.meta.path().get_ident().map(|arg| {
            println!("{}", arg.to_string());
        });
    }
    // let url = attrs.path().get_ident().map(|arg| {
    //     if arg.to_string().contains("url") {
    //         Option::Some(arg)
    //     } else {
    //         None
    //     }
    // }).expect("expected url arg");

    // quote! {
    //     struct #ident #generics {
    //         page_content: String,
    //         #fields
    //     }

    //     impl #ident #generics {
    //         pub fn scrape(&self) {}
    //     }
    // }
    quote! {
        struct Page {}
    }
}

struct AttrArgs {
    attrs: Vec<syn::Attribute>
}

impl syn::parse::Parse for AttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AttrArgs{
            attrs: input.call(Attribute::parse_outer)?
        })
    }
}
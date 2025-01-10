use proc_macro::TokenStream;
use quote::quote;

#[cfg(test)]
mod test {
    use quote::quote;

    #[test]
    fn test_macro(){
        use super::scrape_website_page;

        scrape_website_page(
        quote! {
                    #[scrape_website_page(url="hello")]
                }, 
        quote! {
                #[scrape_website_page(url="hello")]
                struct Page {
                    title: String
                }
            }
        );
    }
}

#[proc_macro_attribute]
pub fn wrap_macro(args: TokenStream, item: TokenStream) -> TokenStream {
    scrape_website_page(args.into(), item.into()).into()
}

fn scrape_website_page(args: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let attrs = syn::parse2::<syn::Meta>(args).unwrap();
    let syn::ItemStruct{
        ident,
        generics,
        fields,
        ..
    } = syn::parse2::<syn::ItemStruct>(item).unwrap();

    println!("{:?}", attrs.path().get_ident().unwrap().to_string());
    
    quote! {
        struct #ident #generics {
            page_content: String,
            #fields
        }

        impl #ident #generics {
            pub fn scrape(&self) {}
        }
    }
}


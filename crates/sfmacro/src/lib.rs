use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn addfields(options: TokenStream, item: TokenStream) -> TokenStream {
    let options = syn::parse_macro_input!(options as syn::Meta);
    let item= syn::parse_macro_input!(item as syn::ItemStruct);

    // println!("{}", options.path().into_token_stream().to_string());
    // todo!()
    let tokens = quote! {
        #item.attrs
        #item.vis struct #item.ident {
            item.fields
        }
    };

    tokens.into()
}

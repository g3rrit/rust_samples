extern crate proc_macro;
extern crate quote;
use proc_macro::TokenStream;
//use syn::{parse_macro_input, ItemEnum};
use quote::quote;


#[proc_macro_derive(Tag)]
pub fn tag_enum_fn(item: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(item as syn::ItemEnum);
  println!("enum id: {}", input.ident);
  
  let id = input.ident;
  let var_count = input.variants.len();
  let var_iter = input.variants.iter();

  let result = quote! {
    use crate::#id::*;
    impl #id {
      fn tag(&self) -> usize { 
        let mut n: usize = 0;
        #( if let #(var_iter.next()) = self { return n; } n += 1; )*
        return n;
      }
      
      fn tag_count(&self) -> usize {
        return #var_count;
      }
    }
  };
  result.into()
}


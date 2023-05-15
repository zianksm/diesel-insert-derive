use std::cell::RefCell;

use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Visibility};

#[proc_macro_attribute]
pub fn auto_insert(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    match item.data {
        Data::Struct(str) => {
            let mut fields_idents = Vec::new();
            let mut pub_fields = Vec::new();
            let mut fields = Vec::new();

            for mut field in str.fields {
                match field.vis {
                    syn::Visibility::Public(_) => {
                        std::mem::replace(&mut field.vis, Visibility::Inherited);
                    }
                    _ => (),
                };
                fields_idents.push(field.ident.clone());
                fields.push(field.clone());
                pub_fields.push(quote::quote! {pub #field});
            }

            let name = item.ident.to_string();
            let name = format!("Insertable{name}")
                .parse::<proc_macro2::TokenStream>()
                .unwrap();

            let attr = attr
                .to_string()
                .parse::<proc_macro2::TokenStream>()
                .unwrap();

            let diesel_args = quote::quote!(#attr);

            let ts = quote::quote! {
                #[derive(Debug,Clone,diesel::Insertable)]
                #[diesel(#diesel_args)]
                pub struct #name{
                    #(#pub_fields),*
                }

                impl #name {
                    pub fn new(#(#fields),*) -> Self{
                        Self{
                            #(#fields_idents),*
                        }
                    }
                }
            };

            println!("{}", ts.to_string());

            ts.into()
        }
        _ => err(item.ident.span(), "can only be used for struct types"),
    }
}

fn err(t_span: proc_macro2::Span, e: &str) -> proc_macro::TokenStream {
    syn::Error::new(t_span, e).to_compile_error().into()
}

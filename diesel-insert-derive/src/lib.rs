use std::{
    collections::{HashSet},
};

use quote::ToTokens;
use syn::{parse::Parser, parse_macro_input, Attribute, Data, DeriveInput, Visibility};

#[proc_macro_attribute]
pub fn exclude(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn auto_insert(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    match item.data.clone() {
        Data::Struct(str) => {
            let exclude = parse_attr(item.attrs.clone()).unwrap();
            let (fields_idents, pub_fields, fields) = parse_fields(str, exclude);

            let name = item.ident.to_string();
            let name = format!("Insertable{name}")
                .parse::<proc_macro2::TokenStream>()
                .unwrap();

            let attr = attr
                .to_string()
                .parse::<proc_macro2::TokenStream>()
                .unwrap();

            let ts = quote::quote! {
                #item
                #[derive(Debug,Clone,diesel::Insertable)]
                #[diesel(#attr)]
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

struct Args(HashSet<String>);
struct ExcludeArgsParser;

impl Parser for ExcludeArgsParser {
    type Output = Vec<String>;

    fn parse2(self, tokens: proc_macro2::TokenStream) -> syn::Result<Self::Output> {
        let args = tokens.to_string();
        let args = args.split_terminator(',').collect::<Vec<&str>>();
        let args = args
            .into_iter()
            .map(|c| c.trim().to_string())
            .collect::<Vec<String>>();

        Ok(args)
    }
}

fn parse_attr(attr: Vec<Attribute>) -> Result<Option<Args>, proc_macro::TokenStream> {
    for attribute in attr {
        if attribute.path().is_ident("exclude") {
            let args = attribute.parse_args_with(ExcludeArgsParser).unwrap();

            let mut map = HashSet::new();
            args.into_iter().for_each(|c| {map.insert(c);});

            return Ok(Some(Args(map)));
        } else {
            let span = attribute.path().get_ident().unwrap().span();
            let msg = format!(
                "unknown identifier! got : {}.\n valid one is : \"exclude\"",
                attribute.path().get_ident().unwrap()
            );

            let e = err(span, &msg);

            return Err(e);
        }
    }

    Ok(None)
}

fn parse_fields(
    str: syn::DataStruct,
    excl_fields: Option<Args>,
) -> (
    Vec<Option<proc_macro2::Ident>>,
    Vec<proc_macro2::TokenStream>,
    Vec<syn::Field>,
) {

    let mut fields_idents = Vec::new();
    let mut pub_fields = Vec::new();
    let mut fields = Vec::new();

    for mut field in str.fields {
        let field_name = field.ident.to_token_stream().to_string();

        if excl_fields.as_ref().is_some() && excl_fields.as_ref().unwrap().0.contains(&field_name) {
            continue;
        }

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

    (fields_idents, pub_fields, fields)
}

fn err(t_span: proc_macro2::Span, e: &str) -> proc_macro::TokenStream {
    syn::Error::new(t_span, e).to_compile_error().into()
}

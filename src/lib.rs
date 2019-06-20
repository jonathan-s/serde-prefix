#![feature(custom_attribute)]

extern crate proc_macro;
#[macro_use]
extern crate syn;
extern crate quote;

use quote::ToTokens;
use syn::{
    AttributeArgs,
    Item,
    ItemStruct,
    ItemEnum,
    Attribute,
    parse_quote
};
use proc_macro::{TokenStream};
use quote::__rt::TokenTree as QuoteToken;


fn check_attributes(attrs: &Vec<Attribute>) {
    let tokens: Vec<_> = attrs.iter()
        .flat_map(|attr| attr.tts.clone().into_iter())
        .map(|tree| match tree {
            QuoteToken::Group(g) => g.stream(),
            _ => panic!("The token tree contains no group")
        })
        .flat_map(|tokenstream| tokenstream.clone().into_iter())
        .filter_map(|tree| match tree {
            QuoteToken::Ident(i) => Some(i),
            _ => None
        })
        .filter(|ident|
            ident.to_string() == "Serialize".to_string()
            || ident.to_string() == "Deserialize".to_string()
        )
        .collect();

    if tokens.len() == 0 {
        panic!("You need to derive from serde to use the macro prefix_all");
    }
}

fn create_attribute(prefix: &str, field_name: &str) -> Attribute {
    let attr_prefix = format!("{}{}", prefix, field_name);
    let attr: Attribute = parse_quote! { #[serde(rename = #attr_prefix)] };
    attr
}


fn handle_enum(token:  &mut ItemEnum, prefix: &str) -> TokenStream {
    let cloned = token.clone();
    check_attributes(&cloned.attrs);

    let variants = &mut token.variants;
    for variant in variants.iter_mut() {
        let field_name = variant.ident.to_string();
        let attr = create_attribute(prefix, &field_name[..]);
        variant.attrs = vec![attr];
    }

    TokenStream::from(token.into_token_stream())
}



fn handle_struct(token: &mut ItemStruct, prefix: &str) -> TokenStream {

    let cloned = token.clone();
    check_attributes(&cloned.attrs);

    let fields = &mut token.fields;
    for field in fields.iter_mut() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let attr = create_attribute(prefix, &field_name[..]);
        field.attrs = vec![attr];
    }

    TokenStream::from(token.into_token_stream())
}


#[proc_macro_attribute]
pub fn prefix_all(attr: TokenStream, item: TokenStream) -> TokenStream {

    let attr_args: Vec<_> = parse_macro_input!(attr as AttributeArgs);
    if attr_args.len() != 1 {
        panic!("prefix_all needs one attribute; the prefix");
    }
    let prefix = Some(&attr_args[0]);
    let prefix = prefix.map(|meta| match meta {
            syn::NestedMeta::Literal(lit) => lit,
            _ => panic!("The attribute is not a string")
        })
        .map(|lit| match lit {
            syn::Lit::Str(string) => string,
            _ => panic!("The attribute is not a string")
        })
        .unwrap().value();


    let mut input = parse_macro_input!(item as Item);
    let tokenstream = match input {
        Item::Enum(ref mut item_enum) => handle_enum(item_enum, &prefix[..]),
        Item::Struct(ref mut item_struct) => handle_struct(item_struct, &prefix[..]),
        _ => panic!("You can't use the macro on this type")
    };

    tokenstream
}

extern crate proc_macro;

use quote::ToTokens;
use syn::{
    AttributeArgs,
    Item,
    ItemStruct,
    ItemEnum,
    Attribute,
    parse_macro_input,
    parse_quote
};
use proc_macro::{TokenStream};


fn create_attribute(prefix: &str, field_name: &str) -> Attribute {
    let attr_prefix = format!("{}{}", prefix, field_name);
    let attr: Attribute = parse_quote! { #[serde(rename = #attr_prefix)] };
    attr
}

fn handle_enum(token:  &mut ItemEnum, prefix: &str) -> TokenStream {

    let variants = &mut token.variants;
    for variant in variants.iter_mut() {
        let field_name = variant.ident.to_string();
        let attr = create_attribute(prefix, &field_name[..]);
        variant.attrs = vec![attr];
    }

    TokenStream::from(token.into_token_stream())
}


fn handle_struct(token: &mut ItemStruct, prefix: &str) -> TokenStream {

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

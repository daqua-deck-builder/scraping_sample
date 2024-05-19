extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Attribute, Variant};

#[proc_macro_attribute]
pub fn model(_args: TokenStream, input: TokenStream) -> TokenStream {
    println!("proc macro executing");
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let attributes = &ast.attrs;

    println!("{}", struct_name);

    let data = match &ast.data {
        syn::Data::Struct(data) => {
            // println!("{:?}", data.fields);
            // data.fieldsには構造体の各フィールドの情報が入っているにゃ
        }
        syn::Data::Enum(data) => {
            for v in data.variants {
                println!("{:?}", v)
            }
            // data.variantsには列挙型の各バリアントの情報が入っているにゃ
        }
        syn::Data::Union(data) => {
            // println!("{:?}", data.fields);
            // data.fieldsにはunionの各フィールドの情報が入っているにゃ
        }
    };


    let gen = quote! {
        // ここで生成するコードを書くにゃ。この例では何もしない。
        #ast
    };

    gen.into()
}
